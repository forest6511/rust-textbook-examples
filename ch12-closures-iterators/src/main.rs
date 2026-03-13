// Rust言語の教科書 — 第12章 サンプルコード
#![allow(clippy::manual_repeat_n)]
#![allow(clippy::unnecessary_fold)]
#![allow(clippy::iter_count)]
#![allow(clippy::useless_vec)]
use std::collections::HashMap;

struct Student {
    name: String,
    score: u32,
}

fn main() {
    // --- クロージャの構文 ---
    fn add_fn(a: i32, b: i32) -> i32 {
        a + b
    }
    let add = |a: i32, b: i32| -> i32 { a + b };
    let add_short = |a, b| a + b;
    println!("{}", add_fn(1, 2));
    println!("{}", add(3, 4));
    println!("{}", add_short(5, 6));

    // --- 環境のキャプチャ ---
    let threshold = 80;
    let scores = vec![65, 92, 78, 88, 55];
    let is_passing = |score: &i32| *score >= threshold;
    let passing: Vec<&i32> = scores.iter().filter(|s| is_passing(s)).collect();
    println!("合格: {passing:?}");
    println!("基準点: {threshold}");

    // --- ミュータブルなキャプチャ ---
    {
        let mut count = 0;
        let mut increment = || {
            count += 1;
            count
        };
        println!("1回目: {}", increment());
        println!("2回目: {}", increment());
        println!("3回目: {}", increment());
    }

    // --- クロージャの3つのトレイト ---
    {
        let name = String::from("Alice");
        let greet = || println!("Hello, {name}");
        greet();
        greet();

        let consume = move || {
            println!("Goodbye, {name}");
            drop(name);
        };
        consume();
    }

    // --- イテレータの基本 ---
    let numbers = vec![10, 20, 30];
    let mut iter = numbers.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    // --- イテレータの生成 ---
    {
        let names = vec![String::from("Alice"), String::from("Bob")];
        for name in names.iter() {
            println!("参照: {name}");
        }
        println!("要素数: {}", names.len());
        for name in names.into_iter() {
            println!("所有: {name}");
        }
    }

    // --- 範囲イテレータ ---
    let sum: i32 = (1..=100).sum();
    println!("1から100の合計: {sum}");
    let zeros: Vec<i32> = std::iter::repeat(0).take(5).collect();
    println!("ゼロ5個: {zeros:?}");

    // --- map ---
    {
        let scores = vec![80, 90, 75, 88];
        let doubled: Vec<i32> = scores.iter().map(|&score| score * 2).collect();
        println!("元の得点: {scores:?}");
        println!("2倍: {doubled:?}");
    }

    // --- filter ---
    {
        let scores = vec![65, 92, 78, 88, 55, 95];
        let high_scores: Vec<&i32> = scores.iter().filter(|&&score| score >= 80).collect();
        println!("80点以上: {high_scores:?}");
    }

    // --- enumerate ---
    let fruits = vec!["りんご", "みかん", "バナナ"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}: {fruit}", i + 1);
    }

    // --- zip ---
    {
        let names = vec!["Alice", "Bob", "Charlie"];
        let scores = vec![85, 92, 78];
        let results: Vec<(&str, &i32)> = names.iter().copied().zip(scores.iter()).collect();
        for (name, score) in &results {
            println!("{name}: {score}点");
        }
    }

    // --- fold ---
    {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = numbers.iter().fold(0, |acc, &n| acc + n);
        println!("合計: {sum}");

        let words = vec!["Hello", " ", "World"];
        let sentence = words.iter().fold(String::new(), |mut acc, &w| {
            acc.push_str(w);
            acc
        });
        println!("{sentence}");
    }

    // --- collect ---
    {
        let data = vec![1, 2, 3, 4, 5, 6];
        let evens: Vec<i32> = data.iter().filter(|&&n| n % 2 == 0).copied().collect();
        println!("偶数: {evens:?}");

        let entries = vec![("Alice", 85), ("Bob", 92)];
        let map: HashMap<&str, i32> = entries.into_iter().collect();
        println!("マップ: {map:?}");
    }

    // --- sumとcount ---
    {
        let prices = vec![250, 480, 120, 890, 350];
        let total: i32 = prices.iter().sum();
        let count = prices.iter().count();
        let average = total as f64 / count as f64;
        println!("合計: {total}円");
        println!("個数: {count}");
        println!("平均: {average:.0}円");
    }

    // --- find と position ---
    {
        let numbers = vec![3, 7, 2, 9, 5, 1];
        let first_even = numbers.iter().find(|&&n| n % 2 == 0);
        println!("最初の偶数: {first_even:?}");
        let pos = numbers.iter().position(|&n| n == 9);
        println!("9の位置: {pos:?}");
        let not_found = numbers.iter().find(|&&n| n > 100);
        println!("100超: {not_found:?}");
    }

    // --- メソッドチェーン ---
    let students = vec![
        Student {
            name: String::from("田中"),
            score: 85,
        },
        Student {
            name: String::from("鈴木"),
            score: 62,
        },
        Student {
            name: String::from("佐藤"),
            score: 91,
        },
        Student {
            name: String::from("高橋"),
            score: 78,
        },
        Student {
            name: String::from("伊藤"),
            score: 45,
        },
    ];
    let passing_names: Vec<&str> = students
        .iter()
        .filter(|s| s.score >= 70)
        .map(|s| s.name.as_str())
        .collect();
    println!("合格者: {passing_names:?}");
    let avg: f64 = students.iter().map(|s| s.score as f64).sum::<f64>() / students.len() as f64;
    println!("平均点: {avg:.1}");

    // --- CSVデータの処理 ---
    let csv_data = "\
name,age,city
Alice,30,Tokyo
Bob,25,Osaka
Charlie,35,Tokyo
Diana,28,Nagoya
Eve,32,Tokyo";
    let tokyo_residents: Vec<&str> = csv_data
        .lines()
        .skip(1)
        .filter(|line| line.ends_with("Tokyo"))
        .map(|line| line.split(',').next().unwrap_or("unknown"))
        .collect();
    println!("東京在住: {tokyo_residents:?}");

    // --- forループとイテレータの比較 ---
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sum_loop = 0;
    for &n in &numbers {
        if n % 2 == 0 {
            sum_loop += n * n;
        }
    }
    let sum_iter: i32 = numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum();
    println!("forループ: {sum_loop}");
    println!("イテレータ: {sum_iter}");
}
