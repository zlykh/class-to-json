use crate::fs_helper::AppContext;
use std::collections::HashMap;
use uuid::Uuid;

const VAL_STRING2: &str = "asd";
const VAL_DEFAULT: &str = "default";
const VAL_LOCAL_DATE: &str = "2021-01-01";
const VAL_LOCAL_DATE_TIME: &str = "2021-01-01T00:00:00";
const VAL_INTEGER: &str = "1";
const VAL_DOUBLE: &str = "1.3";
const VAL_BOOL: &str = "false";

pub enum JsonValue {
    Simple(String),
    Nested(String, bool),
}

pub fn parse_java_text(
    json_src: String,
    app_ctx: &mut AppContext,
    // dict: &HashMap<String, String>,
    // log_lines: &mut Vec<String>,
) -> Vec<String> {
    let mut json_lines = vec![];
    let mut block_comment = false;
    for mut line in json_src.lines() {
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

        let words = line
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .collect::<Vec<_>>();
        if words.is_empty() {
            continue;
        }

        if words.get(1).is_none() || words.get(2).is_none() {
            exit_when_no_pair(app_ctx, &words)
        }

        let (j_type, j_property_name) = (words.get(1).unwrap(), words.get(2).unwrap());

        let mut quotes_needed = true;
        let j_value = match map_type_to_value(j_type, &app_ctx.dict) {
            JsonValue::Nested(text, flag) => {
                quotes_needed = flag;
                text
            }
            JsonValue::Simple(text) => text,
        };

        let json_string = if quotes_needed {
            format!("\"{}\" : \"{}\"", j_property_name, j_value)
        } else {
            format!("\"{}\" : {}", j_property_name, j_value)
        };

        json_lines.push(json_string)
    }
    json_lines
}

fn map_type_to_value(j_type: &str, dict: &HashMap<String, String>) -> JsonValue {
    return match j_type {
        "String" | "string" => JsonValue::Simple(VAL_STRING2.to_string()),
        "Integer" | "int" => JsonValue::Simple(VAL_INTEGER.to_string()),
        "Long" | "long" => JsonValue::Simple(VAL_INTEGER.to_string()),
        "LocalDate" => JsonValue::Simple(VAL_LOCAL_DATE.to_string()),
        "LocalDateTime" => JsonValue::Simple(VAL_LOCAL_DATE_TIME.to_string()),
        "Boolean" | "boolean" => JsonValue::Simple(VAL_BOOL.to_string()),
        "BigDecimal" => JsonValue::Simple(VAL_DOUBLE.to_string()),
        "UUID" => JsonValue::Simple(Uuid::new_v4().to_string()),
        _ => {
            if let Some(val) = dict.get(j_type) {
                JsonValue::Nested(val.clone(), !val.starts_with('{'))
            } else {
                JsonValue::Simple(VAL_DEFAULT.to_string())
            }
        }
    };
}

fn exit_when_no_pair(app_ctx: &mut AppContext, words: &[&str]) {
    // fn exit_when_no_pair(log_lines: &mut Vec<String>, words: &[&str]) {
    let error = format!("Input doesn't produce (type,fieldName) pair {:?}", words);
    println!("{}", &error);
    // log_lines.push(error);
    app_ctx.write_log();
    std::process::exit(1)
}
