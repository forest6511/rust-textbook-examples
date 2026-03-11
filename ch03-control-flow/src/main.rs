// Rust言語の教科書 — 第3章 サンプルコード
fn main() {
    // --- 基本的なif式 ---
    let number = 7;
    if number > 5 {
        println!("5より大きい");
    } else {
        println!("5以下");
    }

    // --- else if ---
    let number = 15;
    if number % 15 == 0 {
        println!("FizzBuzz");
    } else if number % 3 == 0 {
        println!("Fizz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{number}");
    }

    // --- if式で値を返す ---
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("numberは: {number}");

    // --- loop ---
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
    println!("カウント: {count}");

    // --- loopで値を返す ---
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {result}");

    // --- while ---
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("発射!");

    // --- for ---
    let fruits = ["りんご", "みかん", "バナナ"];
    for fruit in &fruits {
        println!("{fruit}");
    }

    // --- for（範囲） ---
    for i in 1..=5 {
        println!("{i}");
    }

    // --- continue ---
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;
        }
        println!("{i}");
    }

    // --- match ---
    let number = 3;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("その他"),
    }

    // --- matchで値を返す ---
    let number = 2;
    let name = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        _ => "不明",
    };
    println!("数字の名前: {name}");

    // --- 複数パターン ---
    let number = 4;
    let kind = match number {
        1 | 3 | 5 | 7 | 9 => "奇数",
        2 | 4 | 6 | 8 | 10 => "偶数",
        _ => "範囲外",
    };
    println!("{number}は{kind}");

    // --- マッチガード ---
    let number = 42;
    match number {
        n if n < 0 => println!("負の数"),
        0 => println!("ゼロ"),
        n if n <= 100 => {
            println!("1〜100の数: {n}");
        }
        _ => println!("100より大きい"),
    }

    // --- 範囲パターン ---
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("成績: {grade}");

    // --- タプルの分解 ---
    let point = (3, -5);
    match point {
        (0, 0) => println!("原点"),
        (x, 0) => println!("x軸上: x={x}"),
        (0, y) => println!("y軸上: y={y}"),
        (x, y) => println!("座標: ({x}, {y})"),
    }

    // --- if let ---
    let value: Option<i32> = Some(42);
    if let Some(n) = value {
        println!("値は: {n}");
    } else {
        println!("値なし");
    }
}
