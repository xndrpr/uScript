use std::fs;
use std::io::Error as IoError;
use serde::{ Serialize, Deserialize };
use toml;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use toml_edit::{Document, Item};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    app: Option<ConfigTomlApp>
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlApp {
    version: Option<String>,
    path: Option<String>,
}


#[derive(Debug)]
pub struct Config {
    pub version: String,
    pub path: String
}

impl Config {
    pub fn new() -> Self {
        /* Create file */
        let metadata_result = fs::metadata("./config.toml");
        let file_exists = match metadata_result {
            Ok(metadata) => metadata.is_file(),
            Err(_) => false,
        };
        if !file_exists {
            let mut toml_doc = Document::new();
            let app = &mut toml_doc["app"];

            app["version"] = Item::Value("0.0.1".into());
            app["path"] = Item::Value("./scripts/".into());

            let mut file = File::create("config.toml").unwrap();
            file.write_all(toml_doc.to_string().as_bytes()).unwrap();
        }


        /* Reading File */
        let config_filepaths: [&str; 2] = [
            "./config.toml",
            "./Config.toml",
        ];
        let mut content : String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml{
                app: None
            }
        });

        let (version, path): (String, String) = match config_toml.app {
            Some(app) => {
            let app_version: String = app.version.unwrap_or_else(|| {
                println!("Missing field username in table app.");
                "unknown".to_owned()
            });

            let app_path: String = app.path.unwrap_or_else(|| {
                println!("Missing field path in table app.");
                "unknown".to_owned()
            });

            (app_version, app_path)
            },
            None => {
            println!("Missing table app.");
            ("unknown".to_owned(), "unknown".to_owned())
            },
        };

        Config {
            version: version,
            path: path
        }
    }

    pub fn modify(&self, version: &str, path_str: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut toml_doc = Document::new();
        let app = &mut toml_doc["app"];
        app["version"] = Item::Value(version.into());
        app["path"] = Item::Value(path_str.into());

        let mut file = File::create("config.toml")?;
        file.write_all(toml_doc.to_string().as_bytes())?;

        Ok(())
    }
}