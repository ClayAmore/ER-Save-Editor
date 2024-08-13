use regex::Regex;

use crate::api::{github::GithubApi, models::releases::Release};

const USER: &str = "ClayAmore";
const REPO: &str = "ER-Save-Editor";

pub struct Updater;

impl Updater {
    // Checks for new app version by fetch latest release from github
    // using GithubApi
    pub fn get_new_version() -> Option<Release> {
        // Fetch latest github release data
        let release = GithubApi::fetch_latest_release(USER, REPO);

        // If something went wrong print the error then return false
        if let Err(err) = &release {
            eprintln!("Failed to fetch latest release.\n Error: {err}");
            return None;
        }

        // Unwrap release result
        let release = release.unwrap();

        // Create a regex to match pattern (nn.nn.nn)
        let re = Regex::new(r"(\d+).(\d+).(\d+)").unwrap();

        // Match regex and extract version sub_match
        if let Some(version_match) = re.captures(&release.tag_name) {
            let remote_major = version_match[1].parse::<u32>().unwrap();
            let remote_minor = version_match[2].parse::<u32>().unwrap();
            let remote_patch = version_match[3].parse::<u32>().unwrap();

            // Local Version
            let local_version = env!("CARGO_PKG_VERSION");
            if let Some(local_version_match) = re.captures(local_version) {
                let local_major = local_version_match[1].parse::<u32>().unwrap();
                let local_minor = local_version_match[2].parse::<u32>().unwrap();
                let local_patch = local_version_match[3].parse::<u32>().unwrap();

                // Check version precedence
                if remote_major > local_major
                    || (remote_major == local_major && remote_minor > local_minor)
                    || (remote_major == local_major
                        && remote_minor == local_minor
                        && remote_patch > local_patch)
                {
                    return Some(release);
                }
            }
        }

        // Default to returning false
        None
    }
}
