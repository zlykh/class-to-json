extern crate dirs;

use clap::Clap;
use uuid::Uuid;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::Local;
use std::error::Error;
/*
    private String myString;

    /**
     * javadoc
     */
    public int myNumber;

    @DynamoDBRangeKey(attributeName = "SK")
    protected Currency curr;

    //comment
    private Clz clz;
 */

#[derive(Clap)]
#[clap(name = "Class to Json", version = "1.0", author = "Dmitry Zlykh")]
struct Opts {
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long)]
    dict: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut log_lines: Vec<String> = Vec::new();
    log_lines.push(Local::now().to_rfc3339());

    let opts: Opts = Opts::parse();

    let log_path: PathBuf = [dirs::home_dir().unwrap().to_str().unwrap(), "class-to-json", "log.txt"]
        .iter().collect();
    let mut path: PathBuf = [dirs::home_dir().unwrap().to_str().unwrap(), "class-to-json", "dict.txt"]
        .iter().collect();
    if let Some(val) = &opts.dict {
        path = PathBuf::from(val)
    }

    log_lines.push(format!("Using dict file at: {:#?}", &path));

    let mut dict: HashMap<String, String> = HashMap::new();
    match fs::read_to_string(&path) {
        Ok(content) => {
            for line in content.trim().split("\r\n").collect::<Vec<_>>() {
                let pair = line.trim().split("=").collect::<Vec<_>>();
                dict.insert(String::from(pair[0]), String::from(pair[1]));
            }
        }
        Err(_) => log_lines.push(format!("Error reading dict file: {:#?}", &path))
    };

    let mut clipboard_content: String = String::new();
    match get_clipboard::<String, formats::Unicode>(formats::Unicode) {
        Ok(x) => clipboard_content = x,
        Err(_) => log_lines.push(format!("Can't access clipboard for read"))
    };
    let json_src = match opts.input {
        Some(cmd_arg) => if cmd_arg.is_empty() { String::new() } else { cmd_arg }
        None => clipboard_content
    };

    log_lines.push(format!("Source text: {:#?}", json_src));
    let lines = json_src.split("\n");
    let mut json_lines = Vec::new();

    let mut block_comment = false;
    for mut line in lines {
        line = line.trim_matches(|c| c == '\r' || c == '\n' || c == ';' || char::is_whitespace(c));
        if line.starts_with('@') || line.starts_with("//") {
            continue;
        }

        if line.starts_with("/*") {
            block_comment = true;
            continue;
        }

        if block_comment {
            if line.starts_with("*/") {
                block_comment = false;
            }
            continue;
        }

        let words = line.split(" ").filter(|w| !w.is_empty()).collect::<Vec<_>>();
        if words.is_empty() {
            continue;
        }

        let j_type = if let Some(x) = words.get(1) { *x } else {
            log_lines.push(format!("Input doesn't produce pair {:#?}", words));
            println!("Input doesn't produce (type,fieldName) pair {:#?}", words);
            std::process::exit(1)
        };

        let j_field = if let Some(x) = words.get(2) { *x } else {
            log_lines.push(format!("Input doesn't produce (type,fieldName) pair {:#?}", words));
            println!("Input doesn't produce (type,fieldName) pair {:#?}", words);
            std::process::exit(1)
        };

        let mut quotes_needed = true;
        let j_value = match j_type {
            "String" | "string" => String::from("asd"),
            "Integer" | "int" => {
                quotes_needed = false;
                String::from("1")
            }
            "Long" | "long" => {
                quotes_needed = false;
                String::from("1.3")
            }
            "LocalDate" => String::from("2021-01-01"),
            "LocalDateTime" => String::from("2021-01-01T00:00:00"),
            "Boolean" | "boolean" => {
                quotes_needed = false;
                String::from("false")
            }
            "BigDecimal" => {
                quotes_needed = false;
                String::from("1.3")
            }
            "UUID" => Uuid::new_v4().to_string(),

            _ => if let Some(val) = dict.get(j_type) {
                quotes_needed = !val.starts_with('{');
                val.clone()
            } else {
                String::from("default")
            }
        };

        let json_string = if quotes_needed {
            format!("\"{}\" : \"{}\"", j_field, j_value)
        } else {
            format!("\"{}\" : {}", j_field, j_value)
        };

        json_lines.push(json_string)
    }
    let json = format!("{{\n{}\n}}", json_lines.join(",\n"));

    log_lines.push(format!("{:#?}", json));
    println!("{}", json);

    match set_clipboard(formats::Unicode, json) {
        Err(_) => log_lines.push(format!("Can't access clipboard for write")),
        Ok(_) => println!("Copied to clipboard!")
    }
    fs::write(log_path, log_lines.join("\r").as_bytes());

    Ok(())
}
