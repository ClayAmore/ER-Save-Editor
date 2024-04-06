pub mod nya {
  use serde::Serialize;
  use serde::Deserialize;
  use std::fs::File;
  use std::path::PathBuf;
  use std::io::Read;


  #[derive(Serialize, Deserialize, Debug)]
  pub struct Stats {
    #[serde(rename = "rl")]
    pub level: u32,
    #[serde(rename = "arc")]
    pub arcane: u32,
    #[serde(rename = "dex")]
    pub dexterity: u32,
    #[serde(rename = "fth")]
    pub faith: u32,
    #[serde(rename = "int")]
    pub intelligence: u32,
    #[serde(rename = "mnd")]
    pub mind: u32,
    #[serde(rename = "str")]
    pub strength: u32,
    #[serde(rename = "vig")]
    pub vigor: u32,
    #[serde(rename = "vit")]
    pub endurance: u32
  }

  #[derive(Serialize, Deserialize, Debug)]
    pub struct Model {
      #[serde(rename = "characterClass")]
      pub character_class: String,
      pub stats: Stats,
    }

    impl Model {
      pub fn from_json(path: PathBuf) -> Self {
        let mut file = File::open(path).expect("Some file contents");
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("something");

        // Deserialize the JSON into the Model struct
        let model: Model = serde_json::from_str(&contents).expect("Could not create Model");

        model
      }
    }
  }