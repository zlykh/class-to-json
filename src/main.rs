mod fs_helper;
mod parser;

extern crate dirs;

use clap::Clap;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use fs_helper::AppContext;
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

   private ClassNotInDict notInDict;
*/

#[derive(Clap)]
#[clap(name = "Class to Json", version = "1.0", author = "Dmitry Zlykh")]
struct Opts {
    #[clap(short, long)]
    input: Option<String>,
    #[clap(short, long)]
    dict: Option<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut app_ctx = AppContext::new();
    app_ctx.load_dict_file(&opts);

    let mut clipboard_content: String = String::new();
    match get_clipboard::<String, formats::Unicode>(formats::Unicode) {
        Ok(x) => clipboard_content = x,
        Err(_) => app_ctx
            .log_lines
            .push("Can't access clipboard for read".to_string()),
    };
    let json_src = match opts.input {
        Some(cmd_arg) => {
            if cmd_arg.is_empty() {
                String::new()
            } else {
                cmd_arg
            }
        }
        None => clipboard_content,
    };

    app_ctx
        .log_lines
        .push(format!("Source text: {:?}", json_src));
    let json_lines = parser::parse_java_text(json_src, &mut app_ctx); //, &app_ctx.dict, log_lines
    let json = format!("{{\n{}\n}}", json_lines.join(",\n"));

    app_ctx.log_lines.push(format!("Result text: {:?}", json));
    println!("{}", json);

    match set_clipboard(formats::Unicode, json) {
        Err(_) => app_ctx
            .log_lines
            .push("Can't access clipboard for write".to_string()),
        Ok(_) => println!("Copied to clipboard!"),
    }

    app_ctx.write_log();
}
