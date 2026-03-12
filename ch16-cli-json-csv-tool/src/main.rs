// Rust言語の教科書 — 第16章 サンプルコード
// dataconv: CSV と JSON の相互変換ツール

use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(
    name = "dataconv",
    about = "CSV と JSON の相互変換ツール"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// CSV を JSON に変換する
    Csv2json {
        /// 入力ファイル（省略時は標準入力）
        input: Option<String>,
        /// 出力ファイル（省略時は標準出力）
        #[arg(short, long)]
        output: Option<String>,
    },
    /// JSON を CSV に変換する
    Json2csv {
        /// 入力ファイル（省略時は標準入力）
        input: Option<String>,
        /// 出力ファイル（省略時は標準出力）
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("エラー: {e}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Commands::Csv2json { input, output } => {
            let content = read_input(&input)?;
            let json = csv_to_json(&content)?;
            write_output(&output, &json)?;
        }
        Commands::Json2csv { input, output } => {
            let content = read_input(&input)?;
            let csv = json_to_csv(&content)?;
            write_output(&output, &csv)?;
        }
    }
    Ok(())
}

fn read_input(
    path: &Option<String>,
) -> Result<String, Box<dyn Error>> {
    match path {
        Some(p) => Ok(fs::read_to_string(p)?),
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}

fn write_output(
    path: &Option<String>,
    content: &str,
) -> Result<(), Box<dyn Error>> {
    match path {
        Some(p) => {
            fs::write(p, content)?;
            eprintln!("{} に書き出しました", p);
        }
        None => {
            io::stdout()
                .write_all(content.as_bytes())?;
        }
    }
    Ok(())
}

fn csv_to_json(
    input: &str,
) -> Result<String, Box<dyn Error>> {
    let mut reader =
        csv::Reader::from_reader(input.as_bytes());
    let mut records: Vec<HashMap<String, String>> =
        Vec::new();
    for result in reader.deserialize() {
        let record: HashMap<String, String> = result?;
        records.push(record);
    }
    let json = serde_json::to_string_pretty(&records)?;
    Ok(json)
}

fn json_to_csv(
    input: &str,
) -> Result<String, Box<dyn Error>> {
    let values: Vec<serde_json::Value> =
        serde_json::from_str(input)?;
    let records = values_to_records(&values)?;

    if records.is_empty() {
        return Ok(String::new());
    }

    let columns = detect_columns(&records);
    records_to_csv(&records, &columns)
}

fn values_to_records(
    values: &[serde_json::Value],
) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>>
{
    let mut records = Vec::new();
    for value in values {
        let obj = value.as_object().ok_or(
            "各要素はJSONオブジェクトである必要があります",
        )?;
        let mut record = HashMap::new();
        for (key, val) in obj {
            let s = match val {
                serde_json::Value::String(s) => {
                    s.clone()
                }
                other => other.to_string(),
            };
            record.insert(key.clone(), s);
        }
        records.push(record);
    }
    Ok(records)
}

fn detect_columns(
    records: &[HashMap<String, String>],
) -> Vec<String> {
    if records.is_empty() {
        return Vec::new();
    }
    let mut columns: Vec<String> =
        records[0].keys().cloned().collect();
    columns.sort();
    columns
}

fn records_to_csv(
    records: &[HashMap<String, String>],
    columns: &[String],
) -> Result<String, Box<dyn Error>> {
    let mut wtr =
        csv::Writer::from_writer(Vec::new());
    wtr.write_record(columns)?;

    for record in records {
        let row: Vec<&str> = columns
            .iter()
            .map(|col| {
                record
                    .get(col)
                    .map(|s| s.as_str())
                    .unwrap_or("")
            })
            .collect();
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    let bytes = wtr.into_inner()?;
    Ok(String::from_utf8(bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_json() {
        let csv = "name,age\nAlice,30\nBob,25";
        let json = csv_to_json(csv).unwrap();
        let parsed: Vec<HashMap<String, String>> =
            serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0]["name"], "Alice");
        assert_eq!(parsed[0]["age"], "30");
    }

    #[test]
    fn test_json_to_csv() {
        let json = r#"[
            {"name": "Alice", "age": "30"},
            {"name": "Bob", "age": "25"}
        ]"#;
        let csv = json_to_csv(json).unwrap();

        assert!(csv.contains("name"));
        assert!(csv.contains("Alice"));
        assert!(csv.contains("Bob"));
    }

    #[test]
    fn test_empty_csv() {
        let csv = "name,age\n";
        let json = csv_to_json(csv).unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_empty_json() {
        let json = "[]";
        let csv = json_to_csv(json).unwrap();
        assert_eq!(csv, "");
    }

    #[test]
    fn test_detect_columns() {
        let mut record = HashMap::new();
        record
            .insert("b".to_string(), "2".to_string());
        record
            .insert("a".to_string(), "1".to_string());
        record
            .insert("c".to_string(), "3".to_string());

        let columns = detect_columns(&[record]);
        assert_eq!(columns, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_json_with_numbers() {
        let json =
            r#"[{"name": "Alice", "age": 30}]"#;
        let csv = json_to_csv(json).unwrap();
        assert!(csv.contains("30"));
    }

    #[test]
    fn test_invalid_json() {
        let json = "not valid json";
        let result = json_to_csv(json);
        assert!(result.is_err());
    }
}
