// Rust言語の教科書 — 第4章 サンプルコード
fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum
}

fn create_greeting() -> String {
    let name = String::from("Rust");
    let greeting = format!("Hello, {name}!");
    greeting
}

fn main() {
    // --- スタックの動き ---
    let x = 5;
    let y = 10;
    let result = add(x, y);
    println!("合計: {result}");

    // --- ヒープを使うString ---
    let s = String::from("hello");
    println!("{s}");

    // --- スタックに置かれる型 ---
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = '🦀';
    let e: (i32, f64) = (1, 2.0);
    println!("{a}, {b}, {c}, {d}, {:?}", e);

    // --- ヒープを使う型 ---
    let s1 = String::from("Rust");
    let s2 = String::from("プログラミング");
    println!("{s1} {s2}");

    // --- 文字列リテラルとStringの違い ---
    let s1: &str = "hello";
    let s2: String = String::from("hello");
    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("等しい? {}", s1 == s2);

    // --- スタック上のデータのコピー ---
    let x = 5;
    let y = x;
    println!("x={x}, y={y}");

    // --- スコープとメモリ ---
    {
        let s = String::from("hello");
        println!("{s}");
    }

    // --- 関数とメモリのライフサイクル ---
    let msg = create_greeting();
    println!("{msg}");
}
