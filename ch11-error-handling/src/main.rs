// Rust言語の教科書 — 第11章 サンプルコード
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

// --- カスタムエラー型（ConfigError） ---
#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        ConfigError::Io(e)
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::Parse(e)
    }
}

fn read_port(path: &str) -> Result<u16, ConfigError> {
    let content = fs::read_to_string(path)?;
    let port: u16 = content.trim().parse()?;
    Ok(port)
}

// --- カスタムエラー型（InputError） ---
#[derive(Debug)]
enum InputError {
    Empty,
    InvalidNumber(ParseIntError),
    OutOfRange(i32),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::Empty => {
                write!(f, "入力が空です")
            }
            InputError::InvalidNumber(e) => {
                write!(f, "数値の変換に失敗: {e}")
            }
            InputError::OutOfRange(n) => {
                write!(f, "範囲外の値: {n}")
            }
        }
    }
}

fn parse_age(input: &str) -> Result<u32, InputError> {
    if input.is_empty() {
        return Err(InputError::Empty);
    }
    let age: i32 = input
        .parse()
        .map_err(InputError::InvalidNumber)?;
    if age < 0 || age > 150 {
        return Err(InputError::OutOfRange(age));
    }
    Ok(age as u32)
}

// --- カスタムエラー型（AppError） ---
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO: {e}"),
            AppError::Parse(e) => {
                write!(f, "変換: {e}")
            }
        }
    }
}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn load_setting(path: &str) -> Result<i32, AppError> {
    let text = std::fs::read_to_string(path)?;
    let value: i32 = text.trim().parse()?;
    Ok(value)
}

// --- Result型の基本 ---
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("ゼロで割ることはできません"))
    } else {
        Ok(a / b)
    }
}

// --- パニックの使いどころ ---
fn create_config(max_connections: u32) -> u32 {
    if max_connections == 0 {
        panic!(
            "max_connectionsは1以上が必要です: {}",
            max_connections
        );
    }
    max_connections
}

// --- 早期リターンパターン ---
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn create_user(
    name: &str,
    age_str: &str,
) -> Result<User, String> {
    if name.is_empty() {
        return Err("名前は必須です".to_string());
    }
    if name.len() > 50 {
        return Err("名前は50文字以内です".to_string());
    }
    let age: u32 = age_str
        .parse()
        .map_err(|_| {
            "年齢は数値で入力してください".to_string()
        })?;
    if age > 150 {
        return Err("年齢が範囲外です".to_string());
    }
    Ok(User {
        name: name.to_string(),
        age,
    })
}

fn main() {
    // --- panic! ---
    let config = create_config(10);
    println!("最大接続数: {config}");

    // --- Result型の基本 ---
    match divide(10.0, 3.0) {
        Ok(result) => println!("結果: {result:.2}"),
        Err(e) => println!("エラー: {e}"),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("結果: {result:.2}"),
        Err(e) => println!("エラー: {e}"),
    }

    // --- matchでResultを処理する ---
    let result = fs::read_to_string("hello.txt");
    match result {
        Ok(content) => println!("内容: {content}"),
        Err(e) => println!("読み込み失敗: {e}"),
    }

    // --- parse() ---
    let num: Result<i32, _> = "42".parse();
    println!("成功: {:?}", num);
    let bad: Result<i32, _> = "abc".parse();
    println!("失敗: {:?}", bad);

    // --- unwrapとexpect ---
    let num: i32 = "42".parse().unwrap();
    println!("数値: {num}");
    let num: i32 =
        "100".parse().expect("数値の変換に失敗");
    println!("数値: {num}");

    // --- ?演算子のチェーン ---
    match read_port("port.txt") {
        Ok(port) => println!("ポート: {port}"),
        Err(e) => println!("エラー: {e:?}"),
    }

    // --- カスタムエラー型（InputError） ---
    let inputs = ["25", "", "abc", "200"];
    for input in inputs {
        match parse_age(input) {
            Ok(age) => println!("年齢: {age}"),
            Err(e) => println!("エラー: {e}"),
        }
    }

    // --- Fromトレイトによる自動変換 ---
    match load_setting("setting.txt") {
        Ok(v) => println!("設定値: {v}"),
        Err(e) => println!("エラー: {e}"),
    }

    // --- 早期リターンパターン ---
    let cases = [
        ("Alice", "30"),
        ("", "25"),
        ("Bob", "abc"),
    ];
    for (name, age) in cases {
        match create_user(name, age) {
            Ok(u) => println!("作成: {:?}", u),
            Err(e) => println!("失敗: {e}"),
        }
    }
}
