use crate::Opts;
use chrono::Local;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub const DICT_FILE_NAME: &str = "dict.txt";
const LOG_FILE_NAME: &str = "log.txt";
const APP_FOLDER: &str = "class-to-json";

#[derive(Default)]
pub struct AppContext {
    home_folder: PathBuf,
    log_file_path: PathBuf,
    pub dict: HashMap<String, String>,
    pub log_lines: Vec<String>,
}

impl AppContext {
    pub fn new() -> Self {
        let home_folder = dirs::home_dir().unwrap();
        AppContext {
            home_folder: home_folder.clone(),
            log_file_path: home_folder.join(APP_FOLDER).join(LOG_FILE_NAME),
            log_lines: vec![Local::now().to_rfc3339()],
            ..Default::default()
        }
    }

    pub fn load_dict_file(&mut self, opts: &Opts) {
        let dict_file_path = if let Some(cmd_arg) = &opts.dict {
            PathBuf::from(cmd_arg)
        } else {
            self.home_folder.join(APP_FOLDER).join(DICT_FILE_NAME)
        };

        self.log_lines
            .push(format!("Using dict file at: {:?}", &dict_file_path));

        match fs::read_to_string(&dict_file_path) {
            Ok(content) => {
                for line in content.trim().lines() {
                    let pair = line.trim().split('=').collect::<Vec<_>>();
                    self.dict
                        .insert(String::from(pair[0]), String::from(pair[1]));
                }
            }
            Err(_) => self
                .log_lines
                .push(format!("Error reading dict file: {:?}", &dict_file_path)),
        };
    }

    pub fn write_log(&self) {
        fs::write(&self.log_file_path, &self.log_lines.join("\n").as_bytes()).unwrap_or_else(
            |_| {
                panic!(
                    "Unable to write log file to {:?}",
                    &self.log_file_path.as_os_str()
                )
            },
        );
    }
}
