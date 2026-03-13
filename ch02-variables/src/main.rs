// Rust言語の教科書 — 第2章 サンプルコード
#![allow(clippy::approx_constant)] // 教育目的で 3.14 リテラルを使用
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];
    for &n in &numbers[1..] {
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

fn main() {
    // --- 変数と束縛 ---
    let x = 5;
    println!("xの値は: {x}");

    // --- ミュータブルな変数 ---
    let mut x = 5;
    println!("xの値は: {x}");
    x = 10;
    println!("xの値は: {x}");

    // --- 定数 ---
    const MAX_POINTS: u32 = 100_000;
    println!("最大ポイント: {MAX_POINTS}");

    // --- シャドーイング ---
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("xの値は: {x}");

    // --- シャドーイング（型の変更） ---
    let spaces = "   ";
    let spaces = spaces.len();
    println!("空白の数: {spaces}");

    // --- 整数型 ---
    let a = 42;
    let b: u8 = 255;
    let c = 1_000_000;
    println!("a={a}, b={b}, c={c}");

    // --- 浮動小数点数型 ---
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x={x}, y={y}");

    // --- 真偽値と文字型 ---
    let is_active: bool = true;
    let letter = 'A';
    let crab = '🦀';
    println!("{is_active}, {letter}, {crab}");

    // --- タプル ---
    let tup: (i32, f64, bool) = (500, 6.4, true);
    let (x, y, z) = tup;
    println!("x={x}, y={y}, z={z}");
    let first = tup.0;
    println!("first={first}");

    // --- 配列 ---
    let months = ["1月", "2月", "3月"];
    println!("最初の月: {}", months[0]);
    println!("配列の長さ: {}", months.len());

    // --- 関数 ---
    let result = add(5, 3);
    println!("5 + 3 = {result}");

    // --- ブロック式 ---
    let y = {
        let x = 3;
        x + 1
    };
    println!("yの値は: {y}");

    // --- 型変換（as） ---
    let x: i32 = 42;
    let y: f64 = x as f64;        // i32 → f64
    let z: u8 = x as u8;          // i32 → u8（範囲内なら安全）
    println!("i32={x}, f64={y}, u8={z}");

    let pi = 3.14_f64;
    let truncated = pi as i32;     // 小数部は切り捨て
    println!("f64={pi}, i32={truncated}");

    // --- 複数の戻り値（タプル） ---
    let nums = [3, 7, 1, 9, 4];
    let (min, max) = min_max(&nums);
    println!("最小: {min}, 最大: {max}");

    // --- 型推論 ---
    let x = 5;
    let y = 2.0;
    let name = "Rust";
    println!("{x}, {y}, {name}");

    // --- 型注釈が必要な例 ---
    let guess: u32 = "42"
        .parse()
        .expect("数値ではありません");
    println!("入力値: {guess}");
}
