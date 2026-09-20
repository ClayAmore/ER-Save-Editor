use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

pub trait ImageSource: Send + Sync {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, String>;
}

pub struct HttpSource;

impl ImageSource for HttpSource {
    fn fetch(&self, url: &str) -> Result<Vec<u8>, String> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .user_agent("er-save-editor")
            .build()
            .map_err(|e| e.to_string())?;

        let response = client.get(url).send().map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            return Err(format!("http {}", response.status()));
        }
        response.bytes().map(|b| b.to_vec()).map_err(|e| e.to_string())
    }
}

// Cache files are keyed by media key (see media::index::key), which is
// namespaced by category so a weapon and an armor that happen to share a
// raw param id cannot collide on the same cache file, and still lets a
// player drop in icons extracted from their own installation and have them
// found.
pub fn default_cache_dir() -> Option<PathBuf> {
    if let Ok(local) = std::env::var("LOCALAPPDATA") {
        let dir = PathBuf::from(local).join("er-save-editor").join("images");
        if std::fs::create_dir_all(&dir).is_ok() {
            return Some(dir);
        }
    }
    let beside = std::env::current_exe().ok()?.parent()?.join("image-cache");
    std::fs::create_dir_all(&beside).ok()?;
    Some(beside)
}

struct Job {
    media_key: u32,
    url: String,
}

pub struct ImageCache {
    jobs: Sender<Job>,
    results: Arc<Mutex<Receiver<(u32, Option<Vec<u8>>)>>>,
    dir: Option<PathBuf>,
}

impl ImageCache {
    pub fn new(source: Arc<dyn ImageSource>, dir: Option<PathBuf>, workers: usize) -> ImageCache {
        let (job_tx, job_rx) = channel::<Job>();
        let (result_tx, result_rx) = channel::<(u32, Option<Vec<u8>>)>();
        let job_rx = Arc::new(Mutex::new(job_rx));

        for _ in 0..workers.max(1) {
            let job_rx = job_rx.clone();
            let result_tx = result_tx.clone();
            let source = source.clone();
            let dir = dir.clone();
            std::thread::spawn(move || loop {
                let job = {
                    let guard = match job_rx.lock() {
                        Ok(guard) => guard,
                        Err(_) => return,
                    };
                    match guard.recv() {
                        Ok(job) => job,
                        Err(_) => return,
                    }
                };

                let path = dir.as_ref().map(|d| d.join(format!("{}.png", job.media_key)));

                // Disk first. An extracted icon wins over the network.
                if let Some(path) = &path {
                    if let Ok(bytes) = std::fs::read(path) {
                        let _ = result_tx.send((job.media_key, Some(bytes)));
                        continue;
                    }
                }

                if job.url.is_empty() {
                    let _ = result_tx.send((job.media_key, None));
                    continue;
                }

                match source.fetch(&job.url) {
                    Ok(bytes) => {
                        if let Some(path) = &path {
                            let _ = std::fs::write(path, &bytes);
                        }
                        let _ = result_tx.send((job.media_key, Some(bytes)));
                    }
                    Err(_) => {
                        let _ = result_tx.send((job.media_key, None));
                    }
                }
            });
        }

        ImageCache {
            jobs: job_tx,
            results: Arc::new(Mutex::new(result_rx)),
            dir,
        }
    }

    // Removes the cached file for `media_key`, if any. Used to heal a
    // poisoned cache entry: a proxy or error page returned with HTTP 200
    // decodes to nothing, and without this it would be re-served from disk
    // forever. Never panics and does not care whether the file existed.
    pub fn discard(&self, media_key: u32) {
        if let Some(dir) = &self.dir {
            let _ = std::fs::remove_file(dir.join(format!("{media_key}.png")));
        }
    }

    pub fn request(&self, media_key: u32, url: &str) {
        let _ = self.jobs.send(Job {
            media_key,
            url: url.to_string(),
        });
    }

    pub fn poll(&self) -> Vec<(u32, Option<Vec<u8>>)> {
        let mut out = Vec::new();
        if let Ok(guard) = self.results.lock() {
            while let Ok(item) = guard.try_recv() {
                out.push(item);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct FakeSource {
        calls: AtomicUsize,
        payload: Option<Vec<u8>>,
    }

    impl ImageSource for FakeSource {
        fn fetch(&self, _url: &str) -> Result<Vec<u8>, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            match &self.payload {
                Some(bytes) => Ok(bytes.clone()),
                None => Err("offline".to_string()),
            }
        }
    }

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("er_cache_test_{tag}"));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    fn drain(cache: &ImageCache, expected: usize) -> Vec<(u32, Option<Vec<u8>>)> {
        let mut got = Vec::new();
        for _ in 0..200 {
            got.extend(cache.poll());
            if got.len() >= expected {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        got
    }

    #[test]
    fn fetches_then_serves_the_same_bytes_from_disk() {
        let dir = temp_dir("hit");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: Some(vec![1, 2, 3]),
        });

        let cache = ImageCache::new(source.clone(), Some(dir.clone()), 2);
        cache.request(1000000, "https://example.test/a.png");
        let got = drain(&cache, 1);
        assert_eq!(got, vec![(1000000, Some(vec![1, 2, 3]))]);
        assert!(dir.join("1000000.png").exists(), "bytes were not cached to disk");

        // A second cache with no working source still answers, from disk.
        let offline = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let second = ImageCache::new(offline.clone(), Some(dir), 2);
        second.request(1000000, "https://example.test/a.png");
        let got = drain(&second, 1);
        assert_eq!(got, vec![(1000000, Some(vec![1, 2, 3]))]);
        assert_eq!(offline.calls.load(Ordering::SeqCst), 0, "disk hit still hit the network");
    }

    #[test]
    fn a_failure_reports_none_without_panicking() {
        let dir = temp_dir("miss");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let cache = ImageCache::new(source, Some(dir), 1);
        cache.request(2009600, "https://example.test/missing.png");
        assert_eq!(drain(&cache, 1), vec![(2009600, None)]);
    }

    #[test]
    fn discard_removes_a_cached_file_and_is_harmless_when_none_exists() {
        let dir = temp_dir("discard");
        std::fs::write(dir.join("1000000.png"), vec![1, 2, 3]).expect("seed icon");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let cache = ImageCache::new(source, Some(dir.clone()), 1);

        assert!(dir.join("1000000.png").exists());
        cache.discard(1000000);
        assert!(!dir.join("1000000.png").exists(), "poisoned cache file was not removed");

        // Discarding a key with no cached file must not panic.
        cache.discard(999999);
    }

    #[test]
    fn a_locally_supplied_icon_is_used_without_any_url() {
        // This is how extracted DLC icons work: the file is simply there.
        let dir = temp_dir("local");
        std::fs::write(dir.join("5200000.png"), vec![9, 9]).expect("seed icon");
        let source = Arc::new(FakeSource {
            calls: AtomicUsize::new(0),
            payload: None,
        });
        let cache = ImageCache::new(source.clone(), Some(dir), 1);
        cache.request(5200000, "");
        assert_eq!(drain(&cache, 1), vec![(5200000, Some(vec![9, 9]))]);
        assert_eq!(source.calls.load(Ordering::SeqCst), 0);
    }
}
