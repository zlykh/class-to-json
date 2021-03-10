use clap::Clap;
use uuid::Uuid;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
/*
    private String gg;
    private Clz clz;
 */

#[derive(Clap)]
#[clap(name = "Class to Json", version = "1.0", author = "Dmitry Zlykh")]
struct Opts {
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long, default_value = ".")]
    dict: String,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();

    let mut dict: HashMap<String, String> = HashMap::new();
    let path: PathBuf = [opts.dict, String::from("dict.txt")].iter().collect();
    println!("path {:#?}", &path);

    match fs::read_to_string(path) {
        Ok(content) => {
            for line in content.trim().split("\r\n").collect::<Vec<_>>() {
                let pair = line.trim().split("=").collect::<Vec<_>>();
                dict.insert(String::from(pair[0]), String::from(pair[1]));
            }
        }
        Err(_) => println!("Error reading dict file")
    };


    let clipboard_content = get_clipboard(formats::Unicode).expect("Can't access clipboard for read");
    let json_src = match opts.input {
        Some(cmd_arg) => if cmd_arg.is_empty() { String::new() } else { cmd_arg }
        None => clipboard_content
    };

    println!("Source text: {:#?}", json_src);
    let lines = json_src.split(";");
    let mut json_lines = Vec::new();

    for mut line in lines {
        line = line.trim_matches(|c| c == '\r' || c == '\n' || char::is_whitespace(c));

        let words = line.split(" ").filter(|w| !w.is_empty()).collect::<Vec<_>>();
        if words.is_empty() {
            continue;
        }

        let j_type = words[1];
        let j_field = words[2];
        let j_value = match j_type {
            "String" | "string" => String::from("asd"),
            "Integer" | "int" => String::from("1"),
            "Long" | "long" => String::from("1"),
            "LocalDate" => String::from("2021-01-01"),
            "LocalDateTime" => String::from("2021-01-01T00:00:00"),
            "Boolean" | "boolean" => String::from("2021-01-01T00:00:00"),
            "BigDecimal" => String::from("1.0"),
            "UUID" => Uuid::new_v4().to_string(),
            _ => if let Some(val) = dict.get(j_type) { val.clone() } else { String::from("default") }
        };
        json_lines.push(format!("\"{}\":\"{}\"", j_field, j_value))
    }
    let json = format!("{{\n{}\n}}", json_lines.join(",\n"));

    //todo error handling on incorrect input

    println!("{}", json);
    println!("Copied to clipboard!");
    set_clipboard(formats::Unicode, json).expect("Can't access clipboard for write");
    Ok(())
}
