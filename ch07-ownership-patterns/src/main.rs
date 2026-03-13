// Rust言語の教科書 — 第7章 サンプルコード
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::unnecessary_literal_unwrap)]
fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

struct User {
    name: String,
    email: String,
}

fn print_user(user: &User) {
    println!("{} <{}>", user.name, user.email);
}

fn update_email(user: &mut User, new_email: &str) {
    user.email = String::from(new_email);
}

fn parse_name(full_name: &str) -> (String, String) {
    match full_name.split_once(' ') {
        Some((first, last)) => (first.to_string(), last.to_string()),
        None => (full_name.to_string(), String::new()),
    }
}

fn print_if_exists(value: &Option<String>) {
    match value.as_ref() {
        Some(s) => println!("値あり: {s}"),
        None => println!("値なし"),
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    // --- パターン1: 関数に渡して戻す（借用） ---
    let name = String::from("rust");
    let upper = to_uppercase(&name);
    println!("{name}は{upper}になった");

    // --- パターン2: ベクタの要素にアクセス ---
    let names = vec![String::from("Alice"), String::from("Bob")];
    let first = &names[0];
    println!("{first}");
    println!("ベクタ: {names:?}");

    // --- パターン3: ベクタのイテレーションと変更 ---
    let mut numbers = vec![1, 2, 3, 4, 5];
    let to_add: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n > 3)
        .map(|&n| n * 10)
        .collect();
    numbers.extend(to_add);
    println!("{numbers:?}");

    // --- パターン4: 構造体と文字列フィールド ---
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    println!("名前: {}, メール: {}", user.name, user.email);

    // --- 構造体を関数に渡す ---
    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    print_user(&user);
    print_user(&user);

    // --- 構造体のフィールドを変更する ---
    let mut user = User {
        name: String::from("Alice"),
        email: String::from("old@example.com"),
    };
    update_email(&mut user, "new@example.com");
    println!("{}: {}", user.name, user.email);

    // --- パターン5: 条件分岐と所有権 ---
    let name = String::from("Alice");
    let greeting = if name.len() > 5 {
        format!("Hello, {name}!")
    } else {
        name.to_string()
    };
    println!("{greeting}");
    println!("元の名前: {name}");

    // --- パターン6: メソッドチェーンと所有権 ---
    let result = String::from("  Hello, Rust!  ")
        .trim()
        .to_lowercase()
        .replace("rust", "World");
    println!("結果: '{result}'");

    // --- パターン7: 関数から複数の値を返す ---
    let (first, last) = parse_name("Taro Yamada");
    println!("名: {first}, 姓: {last}");

    // --- パターン8: Optionと所有権 ---
    let name: Option<String> = Some(String::from("Alice"));
    print_if_exists(&name);
    println!("再利用: {name:?}");

    // --- unwrap_or_default() ---
    let name: Option<String> = None;
    let display = name.unwrap_or_default();
    println!("名前: '{display}'");

    let name2: Option<String> = Some(String::from("Bob"));
    let display2 = name2.unwrap_or_default();
    println!("名前: '{display2}'");

    // --- パターン9: forループと所有権 ---
    let names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];
    for name in &names {
        println!("Hello, {name}!");
    }
    println!("全員: {names:?}");

    // --- 所有権を消費するイテレーション ---
    let names = vec![String::from("Alice"), String::from("Bob")];
    for name in names {
        println!("Goodbye, {name}!");
    }

    // --- String と &str の使い分けガイド ---
    let name = String::from("Rust");
    let msg = greet(&name);
    let msg2 = greet("World");
    println!("{msg}");
    println!("{msg2}");
}
