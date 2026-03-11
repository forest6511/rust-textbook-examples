// Rust言語の教科書 — 第6章 サンプルコード
fn print_string(s: &str) {
    println!("{s}");
}

fn add_world(s: &mut String) {
    s.push_str(", world");
}

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn count_chars(s: &str) -> usize {
    s.len()
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    // --- 不変参照の基本 ---
    let msg = String::from("hello");
    print_string(&msg);
    println!("msgはまだ使える: {msg}");

    // --- デリファレンス ---
    let x = 5;
    let r = &x;
    let y = *r + 10;
    println!("r={r}, y={y}");

    // --- &strを使うイディオム ---
    let msg = String::from("hello");
    print_string(&msg);
    print_string("world");

    // --- 複数の不変参照 ---
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1}, {r2}");

    // --- ミュータブル参照 ---
    let mut msg = String::from("hello");
    add_world(&mut msg);
    println!("{msg}");

    // --- NLL ---
    let mut s = String::from("hello");
    let r1 = &s;
    println!("{r1}");
    let r2 = &mut s;
    r2.push_str(", world");
    println!("{r2}");

    // --- 文字列スライス ---
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");

    // --- 範囲指定の省略 ---
    let s = String::from("hello");
    let s1 = &s[0..5];
    let s2 = &s[..5];
    let s3 = &s[0..];
    let s4 = &s[..];
    println!("{s1}, {s2}, {s3}, {s4}");

    // --- 配列のスライス ---
    let nums = [1, 2, 3, 4, 5];
    let total = sum(&nums);
    let partial = sum(&nums[1..4]);
    println!("合計: {total}, 部分合計: {partial}");

    // --- clone()から借用へのリファクタリング ---
    let msg = String::from("hello, world");
    let len = count_chars(&msg);
    println!("{msg}は{len}文字");

    // --- ライフタイム注釈 ---
    let s1 = String::from("Rust");
    let s2 = String::from("Go");
    let result = longer(&s1, &s2);
    println!("長い方: {result}");

    // --- ライフタイム省略規則 ---
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("最初の単語: {word}");
}
