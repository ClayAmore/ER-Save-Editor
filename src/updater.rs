pub mod updater {
  use reqwest::blocking::{Client, get};
  use serde::Deserialize;
  use std::env;
  use std::fs;
  use std::process::Command;
  use std::io::copy;
  use tempfile::NamedTempFile;

  impl Default for UpdaterApp {
      fn default() -> Self {
          Self { 
              release_notes: "Release notes for the latest version...".into(),
              update_confirmed: None,
              browser_download_url: String::from(""),
          }
      }
  }

  impl eframe::App for UpdaterApp {
      fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
          egui::CentralPanel::default().show(ctx, |ui| {
              // Use a vertical layout to center the content in the middle
              ui.vertical_centered(|ui| {
                  // Create a larger font for the buttons and label
                  let big_text_style = egui::FontFamily::Monospace; // Choose an appropriate style
                  let font_id = egui::FontId::new(20.0, big_text_style); // Set the desired font size
          
                  // Apply the larger font to the buttons
                  if ui.add_sized(
                      egui::vec2(100.,100.),
                      egui::Button::new(
                          egui::RichText::new("Update and re-launch").font(font_id.clone())
                              )
                              .wrap(false)
                  ).clicked() {
                      self.update_confirmed = Some(true);
                      ctx.request_repaint();
                  };
                  ui.add_space(10.0); // Add some space between the buttons
                  if ui.add_sized(
                      egui::vec2(100.,100.),
                      egui::Button::new(
                          egui::RichText::new("Skip Update").font(font_id.clone())
                              )
                          .wrap(false)
                  ).clicked() {
                      self.update_confirmed = Some(false);
                      ctx.request_repaint();
                  };
          
                  ui.add_space(20.0); // Add some space before the label
          
                  // Apply the larger font to the label
                  ui.label(egui::RichText::new(&self.release_notes).font(font_id));
              });
          });

          if let Some(confirmed) = self.update_confirmed {
              if confirmed {
                  // Proceed with the update
                  println!("User confirmed update.");
              
                  // Download the latest binary and replace the current executable
                  let current_exe_path = Self::download_and_replace_current_binary(&self.browser_download_url);

                  match current_exe_path{
                      Ok(path) => {
                          // Optionally, restart the program after updating
                          Command::new(path).spawn().expect("Program did not restart correctly");
                      },
                      Err(e) => {
                          println!("Program did not restart correctly: {}", e)
                      }
                  }
                  
                  
              } else {
                  // Cancel the update
                  println!("User canceled update.");
              }
              ctx.send_viewport_cmd(egui::ViewportCommand::Close)
          }
      }
  }



  #[derive(Deserialize)]
  struct Release {
      tag_name: String,
      body: String,
      assets: Vec<ReleaseAsset>,
      prerelease: bool
  }


  #[derive(Deserialize)]
  struct ReleaseAsset {
      browser_download_url: String,
  }


  fn fetch_latest_release() -> Result<Release, reqwest::Error> {
      let url = "https://api.github.com/repos/clayamore/er-save-editor/releases/latest";
      let client = Client::new();
      let release = client.get(url)
          .header("User-Agent", "request")
          .send()?
          .json::<Release>()?;

      Ok(release)
  }

  pub struct UpdaterApp {
    release_notes: String,
    browser_download_url: String,
    update_confirmed: Option<bool>,
  }

  impl UpdaterApp {
    fn download_and_replace_current_binary(download_url: &str) -> Result<String, Box<dyn std::error::Error>> {
      let current_exe_path = env::current_exe().expect("Could not find the currently running exe").display().to_string();

      let response = get(download_url)?;
      let mut tmp_file = NamedTempFile::new()?;
      copy(&mut response.bytes()?.as_ref(), &mut tmp_file)?;

      fs::rename(tmp_file.path(), current_exe_path.clone())?;

      Ok(current_exe_path)
    }

    pub fn check_for_updates() -> Result<(), Box<dyn std::error::Error>> {
      let local_version = env!("CARGO_PKG_VERSION"); // Automatically gets the version
  
      let latest_release = fetch_latest_release()?;
      if latest_release.prerelease {
          return Ok(());
      }
      let latest_version = latest_release.tag_name.trim_start_matches('v').to_owned();
      let latested_release_notes = latest_release.body;
      if latest_version != local_version {
          println!("An update is available! Your version: {}, latest version: {}", local_version, latest_version);
          let options = eframe::NativeOptions {
              viewport: egui::ViewportBuilder::default().with_inner_size([1000., 800.]),
              ..Default::default()
          };
          
          if let Some(asset) = latest_release.assets.iter().find(|a| a.browser_download_url.ends_with("er-save-editor.exe")) {
              let mut app_updater = Self::default();
              app_updater.browser_download_url = asset.browser_download_url.to_string();
              app_updater.release_notes = latested_release_notes.to_string();
  
              match eframe::run_native(
                  "App Update",
                  options,
                  Box::new(|_cc| Box::new(app_updater))
              ) {
                  Ok(_result) => {
  
                  },
                  Err(e) => {
                      println!("{}", e)
                  }
              }
              
          } else {
              println!("No suitable binary found in the latest release.");
          }
      } else {
          println!("Your program is up to date.");
      }
  
      Ok(())
  }
  }
}