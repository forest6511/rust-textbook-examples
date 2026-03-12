// Rust言語の教科書 — 第15章 サンプルコード
// rustfind: ファイル検索CLIツール

use clap::Parser;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

/// ファイル検索ツール
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// ファイル名に含まれるパターン
    #[arg(short, long)]
    name: Option<String>,

    /// ファイル内容で検索するキーワード
    #[arg(short, long)]
    content: Option<String>,

    /// 検索対象のディレクトリ
    path: String,
}

#[derive(Debug)]
enum AppError {
    Io(io::Error),
    InvalidArgs(String),
}

impl fmt::Display for AppError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            AppError::Io(e) => {
                write!(f, "IOエラー: {e}")
            }
            AppError::InvalidArgs(msg) => {
                write!(f, "引数エラー: {msg}")
            }
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

fn matches_name(
    path: &Path,
    pattern: &str,
) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.contains(pattern))
        .unwrap_or(false)
}

fn search_content(
    path: &Path,
    keyword: &str,
) -> Vec<(usize, String)> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(keyword))
        .map(|(i, line)| (i + 1, line.to_string()))
        .collect()
}

fn search_files(args: &Args) {
    let mut found = false;

    for entry in WalkDir::new(&args.path) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("走査エラー: {e}");
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        if let Some(ref pat) = args.name {
            if !matches_name(path, pat) {
                continue;
            }
        }

        if let Some(ref keyword) = args.content {
            let matches =
                search_content(path, keyword);
            if !matches.is_empty() {
                found = true;
                println!("{}:", path.display());
                for (num, line) in &matches {
                    println!("  {num}: {line}");
                }
            }
        } else {
            found = true;
            println!("{}", path.display());
        }
    }

    if !found {
        println!(
            "一致するファイルはありませんでした"
        );
    }
}

fn run(args: &Args) -> Result<(), AppError> {
    if args.name.is_none()
        && args.content.is_none()
    {
        return Err(AppError::InvalidArgs(
            "--name または --content を\
             指定してください"
                .to_string(),
        ));
    }

    let path = Path::new(&args.path);
    if !path.exists() {
        return Err(AppError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "ディレクトリが見つかりません: {}",
                args.path
            ),
        )));
    }

    search_files(args);
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("エラー: {e}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_matches_name_found() {
        let path = Path::new("/tmp/hello.rs");
        assert!(matches_name(path, "hello"));
    }

    #[test]
    fn test_matches_name_not_found() {
        let path = Path::new("/tmp/hello.rs");
        assert!(!matches_name(path, "world"));
    }

    #[test]
    fn test_matches_name_extension() {
        let path = Path::new("/tmp/main.rs");
        assert!(matches_name(path, ".rs"));
    }

    #[test]
    fn test_matches_name_partial() {
        let path = Path::new("/tmp/test_helper.rs");
        assert!(matches_name(path, "test"));
        assert!(matches_name(path, "helper"));
    }

    #[test]
    fn test_search_content_found() {
        let dir = std::env::temp_dir();
        let file = dir.join("rustfind_test.txt");
        fs::write(
            &file,
            "hello world\nfoo bar\nhello rust\n",
        )
        .unwrap();

        let results =
            search_content(&file, "hello");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, 1);
        assert_eq!(results[1].0, 3);

        fs::remove_file(&file).ok();
    }

    #[test]
    fn test_search_content_not_found() {
        let dir = std::env::temp_dir();
        let file = dir.join("rustfind_test2.txt");
        fs::write(&file, "aaa\nbbb\nccc\n")
            .unwrap();

        let results =
            search_content(&file, "xyz");
        assert!(results.is_empty());

        fs::remove_file(&file).ok();
    }

    #[test]
    fn test_run_no_args_error() {
        let args = Args {
            name: None,
            content: None,
            path: ".".to_string(),
        };
        let result = run(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_nonexistent_path() {
        let args = Args {
            name: Some("test".to_string()),
            content: None,
            path: "/nonexistent_dir".to_string(),
        };
        let result = run(&args);
        assert!(result.is_err());
    }
}
