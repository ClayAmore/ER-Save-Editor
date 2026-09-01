use std::collections::HashMap;
use std::sync::Arc;

use eframe::egui;

use super::cache::{default_cache_dir, HttpSource, ImageCache};
use super::index;

enum State {
    Requested,
    Ready(egui::TextureHandle),
    // Never retried. This is what stops an offline list asking once a frame.
    Failed,
}

pub struct ItemTextures {
    cache: ImageCache,
    state: HashMap<u32, State>,
}

impl ItemTextures {
    pub fn new() -> ItemTextures {
        ItemTextures {
            cache: ImageCache::new(Arc::new(HttpSource), default_cache_dir(), 4),
            state: HashMap::new(),
        }
    }

    pub fn state_count(&self) -> usize {
        self.state.len()
    }

    // Called once per frame, before anything asks for a texture.
    pub fn poll(&mut self, ctx: &egui::Context) {
        for (media_key, bytes) in self.cache.poll() {
            let next = match bytes.and_then(|bytes| decode(&bytes)) {
                Some((size, pixels)) => {
                    let image = egui::ColorImage { size, pixels };
                    State::Ready(ctx.load_texture(
                        format!("item_{media_key}"),
                        image,
                        egui::TextureOptions::LINEAR,
                    ))
                }
                None => State::Failed,
            };
            self.state.insert(media_key, next);
        }
    }

    // `media_key` is namespaced by category (media::index::key). Only the
    // caller knows which of the five equipment tables a param id came from,
    // so it computes the key before calling here.
    pub fn texture(&mut self, media_key: u32) -> Option<egui::TextureHandle> {
        match self.state.get(&media_key) {
            Some(State::Ready(handle)) => return Some(handle.clone()),
            Some(_) => return None,
            None => {}
        }

        // An item the index does not know still gets a slot, so the miss is
        // remembered and never asked about again.
        let url = index::entry(media_key).map(|e| e.image_url.clone()).unwrap_or_default();
        self.cache.request(media_key, &url);
        self.state.insert(media_key, State::Requested);
        None
    }
}

fn decode(bytes: &[u8]) -> Option<([usize; 2], Vec<egui::Color32>)> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = image.dimensions();
    let pixels = image
        .into_raw()
        .chunks_exact(4)
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    Some(([width as usize, height as usize], pixels))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_item_with_no_index_entry_is_never_requested_twice() {
        let mut textures = ItemTextures::new();
        // 1 is not a real media key in the index.
        assert!(textures.texture(1).is_none());
        assert!(textures.texture(1).is_none());
        assert_eq!(textures.state_count(), 1, "the miss was not remembered");
    }
}
