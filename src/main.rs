use clap::Parser;
use colored::*;
use serde_json::{Value, Map};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Processes and colorizes JSON input.", long_about = None)]
struct Args {
  #[arg(help = "JSON string to process. Make sure to enclose the JSON in single quotes (' ').")]
  json_input: String,
}

fn main() {
  let args = Args::parse();

  let processed_json = process_json(&args.json_input).unwrap_or_else(|err| {
    eprintln!("Error processing JSON: {}", err);
    std::process::exit(1);
  });

  println!("{}", colorize_json(&processed_json));
}

fn process_json(json_input: &str) -> Result<String, serde_json::Error> {
  let placeholder_regex = Regex::new(r"\[([A-Za-z]+)\]").unwrap();
  let mut preprocessed_input = placeholder_regex.replace_all(json_input, "\"[$1]\"").to_string();

  let date_regex = Regex::new(r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z)").unwrap();
  preprocessed_input = date_regex.replace_all(&preprocessed_input, "\"$1\"").to_string();

  let bigint_regex = Regex::new(r"(\d+)n").unwrap();
  preprocessed_input = bigint_regex.replace_all(&preprocessed_input, "$1").to_string();

  let undefined_regex = Regex::new(r"\bundefined\b").unwrap();
  preprocessed_input = undefined_regex.replace_all(&preprocessed_input, "null").to_string();

  let value: Value = serde_json::from_str(&preprocessed_input)?;

  let json_string = serde_json::to_string_pretty(&value)?;

  Ok(json_string)
}

fn colorize_json(json_str: &str) -> String {
  let parsed_json: Value = serde_json::from_str(json_str).unwrap();
  format_json_value(&parsed_json)
}

fn format_json_value(value: &Value) -> String {
  match value {
    Value::Object(map) => format_object(map),
    Value::Array(vec) => format_array(vec),
    Value::String(s) => s.green().to_string(),
    Value::Number(n) => n.to_string().yellow().to_string(),
    Value::Bool(b) => b.to_string().bright_blue().to_string(),
    Value::Null => "null".magenta().to_string()
  }
}

fn format_object(map: &Map<String, Value>) -> String {
  let mut result = String::from("{\n");
  for (k, v) in map {
    result.push_str(&format!("  \"{}\": {},\n", k.green(), format_json_value(v).replace("\n", "\n  ")));
  }
  if !map.is_empty() {
    result.truncate(result.len() - 2);
  }
  result.push_str("\n}");
  result
}

fn format_array(vec: &Vec<Value>) -> String {
  let mut result = String::from("[\n");
  for item in vec {
    result.push_str(&format!("  {},\n", format_json_value(item).replace("\n", "\n  ")));
  }
  if !vec.is_empty() {
    result.truncate(result.len() - 2);
  }
  result.push_str("\n]");
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_process_and_colorize_json_with_object_placeholder() {
    let json_input = r#"{"key": [Object]}"#;
    let expected_output = "{\n  \"\u{1b}[32mkey\u{1b}[0m\": \u{1b}[32m[Object]\u{1b}[0m\n}".to_string();

    let processed_json = process_json(json_input).expect("Failed to process JSON");
    let colorized_json = colorize_json(&processed_json);

    assert_eq!(colorized_json, expected_output, "JSON output did not match expected colorized output");
  }

  #[test]
  fn test_process_and_colorize_json_with_array_placeholder() {
    let json_input = r#"{"key": [Array]}"#;
    let expected_output = "{\n  \"\u{1b}[32mkey\u{1b}[0m\": \u{1b}[32m[Array]\u{1b}[0m\n}".to_string();

    let processed_json = process_json(json_input).expect("Failed to process JSON");
    let colorized_json = colorize_json(&processed_json);

    assert_eq!(colorized_json, expected_output, "JSON output did not match expected colorized output");
  }

  #[test]
  fn test_process_and_colorize_json_with_undefined_placeholder() {
    let json_input = r#"{"key": undefined}"#;
    let expected_output = "{\n  \"\u{1b}[32mkey\u{1b}[0m\": \u{1b}[35mnull\u{1b}[0m\n}".to_string();

    let processed_json = process_json(json_input).expect("Failed to process JSON");
    let colorized_json = colorize_json(&processed_json);

    assert_eq!(colorized_json, expected_output, "JSON output did not match expected colorized output");
  }

  #[test]
  fn test_process_and_colorize_json_with_bool() {
    let json_input = r#"{"key": true}"#;
    let expected_output = "{\n  \"\u{1b}[32mkey\u{1b}[0m\": \u{1b}[94mtrue\u{1b}[0m\n}".to_string();

    let processed_json = process_json(json_input).expect("Failed to process JSON");
    let colorized_json = colorize_json(&processed_json);

    assert_eq!(colorized_json, expected_output, "JSON output did not match expected colorized output");
  }

  #[test]
  fn test_process_and_colorize_json_with_number() {
    let json_input = r#"{"key": 123}"#;
    let expected_output = "{\n  \"\u{1b}[32mkey\u{1b}[0m\": \u{1b}[33m123\u{1b}[0m\n}".to_string();

    let processed_json = process_json(json_input).expect("Failed to process JSON");
    let colorized_json = colorize_json(&processed_json);

    assert_eq!(colorized_json, expected_output, "JSON output did not match expected colorized output");
  }
}
