// Rust言語の教科書 — 第9章 サンプルコード
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]
#![allow(clippy::unwrap_or_default)]
use std::collections::HashMap;

fn average(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

fn word_count(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    counts
}

fn group_by_length<'a>(words: &[&'a str]) -> HashMap<usize, Vec<&'a str>> {
    let mut groups: HashMap<usize, Vec<&str>> = HashMap::new();

    for &word in words {
        groups
            .entry(word.len())
            .or_insert_with(Vec::new)
            .push(word);
    }
    groups
}

fn main() {
    // --- ベクタの作成 ---
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("{numbers:?}");

    let fruits = vec!["りんご", "みかん", "バナナ"];
    println!("{fruits:?}");

    // --- 要素へのアクセス ---
    let colors = vec!["赤", "青", "緑"];
    let first = &colors[0];
    println!("最初の色: {first}");
    match colors.get(5) {
        Some(color) => println!("色: {color}"),
        None => println!("インデックス5は範囲外"),
    }

    // --- ベクタの操作 ---
    let mut nums = vec![3, 1, 4, 1, 5, 9];
    println!("長さ: {}", nums.len());
    let last = nums.pop();
    println!("取り出し: {last:?}");
    nums.sort();
    println!("ソート後: {nums:?}");
    println!("4を含む? {}", nums.contains(&4));
    nums.insert(0, 0);
    println!("挿入後: {nums:?}");

    // --- ベクタのイテレーション ---
    let mut scores = vec![80, 90, 75];
    for score in &scores {
        println!("得点: {score}");
    }
    for score in &mut scores {
        *score += 10;
    }
    println!("加算後: {scores:?}");
    let total: i32 = scores.into_iter().sum();
    println!("合計: {total}");

    // --- ベクタとスライス ---
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("全体の平均: {:.1}", average(&data));
    println!("前半の平均: {:.1}", average(&data[..3]));

    // --- HashMapの作成 ---
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    scores.insert(String::from("Charlie"), 92);
    println!("{scores:?}");

    // --- 値の取得 ---
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 85);
    match scores.get("Alice") {
        Some(score) => println!("Aliceの得点: {score}"),
        None => println!("Aliceは見つかりません"),
    }
    let unknown = scores.get("Dave");
    println!("Daveの得点: {unknown:?}");

    // --- 値の更新 ---
    let mut map = HashMap::new();
    map.insert("key", 1);
    map.insert("key", 2);
    println!("上書き: {:?}", map.get("key"));
    map.entry("key").or_insert(999);
    map.entry("new").or_insert(999);
    println!("or_insert後: {map:?}");
    let count = map.entry("key").or_insert(0);
    *count += 10;
    println!("更新後: {map:?}");

    // --- 単語カウント ---
    let text = "the cat sat on the mat the cat";
    let counts = word_count(text);
    for (word, count) in &counts {
        println!("{word}: {count}回");
    }

    // --- HashMapと所有権 ---
    let key = String::from("name");
    let value = String::from("Alice");
    let mut map = HashMap::new();
    map.insert(key, value);
    let mut map2 = HashMap::new();
    map2.insert("name", "Bob");
    println!("{map:?}");
    println!("{map2:?}");

    // --- Stringの内部構造 ---
    let mut s = String::new();
    let s2 = String::from("hello");
    let s3 = "hello".to_string();
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");
    println!("{s}");
    println!("長さ: {}バイト", s.len());
    println!("容量: {}バイト", s.capacity());
    println!("s2={s2}, s3={s3}");

    // --- UTF-8と日本語 ---
    let s = String::from("こんにちは");
    println!("バイト数: {}", s.len());
    println!("文字数: {}", s.chars().count());
    for c in s.chars() {
        print!("{c} ");
    }
    println!();
    for b in s.bytes().take(6) {
        print!("{b:02x} ");
    }
    println!("...");

    // --- 文字列の結合 ---
    let hello = String::from("Hello");
    let world = String::from(", World");
    let msg = format!("{hello}{world}!");
    println!("{msg}");
    let greeting = hello + &world;
    println!("{greeting}");
    println!("worldはまだ使える: {world}");

    // --- 文字列の分割と検索 ---
    let csv = "Alice,30,Tokyo";
    let fields: Vec<&str> = csv.split(',').collect();
    println!("{fields:?}");
    println!("Tokyoを含む? {}", csv.contains("Tokyo"));
    println!("Aliceで始まる? {}", csv.starts_with("Alice"));
    let replaced = csv.replace(",", " | ");
    println!("{replaced}");
    let padded = "  hello  ";
    println!("トリム: '{}'", padded.trim());

    // --- Stringと&strの変換 ---
    let s1: String = "hello".to_string();
    let s2: String = String::from("hello");
    let s3: &str = &s1;
    let s4: &str = s1.as_str();
    println!("{s1}, {s2}, {s3}, {s4}");

    // --- コレクションの組み合わせ ---
    let words = vec!["cat", "dog", "fish", "bird", "ant"];
    let groups = group_by_length(&words);
    for (len, words) in &groups {
        println!("{len}文字: {words:?}");
    }
}
