// Rust言語の教科書 — 第5章 サンプルコード
fn print_string(s: String) {
    println!("{s}");
}

fn print_number(n: i32) {
    println!("{n}");
}

fn create_greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

fn add_exclamation(mut s: String) -> String {
    s.push('!');
    s
}

fn describe(n: i32) -> String {
    match n {
        0 => String::from("ゼロ"),
        1..=9 => format!("1桁の数: {n}"),
        _ => format!("大きな数: {n}"),
    }
}

fn main() {
    // --- 所有権のルール ---
    let s = String::from("hello");
    println!("{s}");

    // --- ムーブ ---
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s2}");

    // --- clone() ---
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={s1}, s2={s2}");

    // --- Copyトレイト ---
    let x = 5;
    let y = x;
    println!("x={x}, y={y}");

    // --- CopyとCloneの違い ---
    let a: i32 = 42;
    let b = a;
    println!("a={a}, b={b}");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={s1}, s2={s2}");

    // --- 関数に値を渡す ---
    let msg = String::from("hello");
    print_string(msg);

    let x = 42;
    print_number(x);
    println!("xはまだ使える: {x}");

    // --- clone()で関数呼び出し後も使い続ける ---
    let msg = String::from("hello");
    print_string(msg.clone());
    println!("msgはまだ使える: {msg}");

    // --- 関数から値を返す ---
    let greeting = create_greeting("Rust");
    println!("{greeting}");

    // --- 所有権を返して再利用する ---
    let msg = String::from("hello");
    let msg = add_exclamation(msg);
    println!("{msg}");

    // --- スコープの入れ子と所有権 ---
    let outer = String::from("outer");
    {
        let inner = String::from("inner");
        println!("{outer}, {inner}");
    }
    println!("{outer}");

    // --- 内側のスコープから外側に所有権をムーブ ---
    let s;
    {
        let inner = String::from("hello");
        s = inner;
    }
    println!("{s}");

    // --- if式と所有権 ---
    let greeting = if true {
        String::from("Hello")
    } else {
        String::from("Goodbye")
    };
    println!("{greeting}");

    // --- matchと所有権 ---
    let msg = describe(42);
    println!("{msg}");
}
