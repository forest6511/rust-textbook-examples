// Rust言語の教科書 — 第10章 サンプルコード
#![allow(dead_code)]
#![allow(clippy::approx_constant)]
use std::fmt;

// --- トレイトの定義 ---
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("({}さんの記事)", self.summarize_author())
    }
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

struct NewsFlash {
    author: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{} ({})", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        self.username.clone()
    }

    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

impl Summary for NewsFlash {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    // summarize()はデフォルト実装を使う
}

// --- ジェネリックな関数 ---
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] {
        if item > max {
            max = item;
        }
    }
    max
}

// --- ジェネリックな構造体 ---
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

// --- トレイト境界 ---
fn print_info<T: fmt::Display>(item: &T) {
    println!("値: {item}");
}

fn print_debug_and_display<T: fmt::Display + fmt::Debug>(item: &T) {
    println!("Display: {item}");
    println!("Debug: {item:?}");
}

// --- where句 ---
fn process<T, U>(t: &T, u: &U) -> String
where
    T: fmt::Display + Clone,
    U: fmt::Debug,
{
    format!("{t} / {u:?}")
}

// --- Display と Debug ---
#[derive(Debug)]
struct Temperature {
    celsius: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

// --- Clone と Copy ---
#[derive(Debug, Clone)]
struct Config {
    name: String,
    value: i32,
}

// --- PartialEq と PartialOrd ---
#[derive(Debug, PartialEq, PartialOrd)]
struct Score {
    value: u32,
}

// --- トレイトオブジェクト ---
trait Animal {
    fn speak(&self) -> &str;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}
struct Cat {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> &str {
        "ワン"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn speak(&self) -> &str {
        "ニャー"
    }
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    // --- トレイトの基本 ---
    let article = Article {
        title: String::from("Rustの所有権"),
        author: String::from("田中"),
    };
    println!("{}", article.summarize());

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust 1.75リリース"),
    };
    println!("{}", tweet.summarize());

    // --- デフォルト実装 ---
    let flash = NewsFlash {
        author: String::from("鈴木"),
    };
    println!("{}", flash.summarize());

    // --- ジェネリックな関数 ---
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大の数値: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大の文字: {}", largest(&chars));

    // --- ジェネリックな構造体 ---
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.5, 3.7);
    println!("整数: {:?}", int_point);
    println!("小数: {:?}", float_point);

    // --- トレイト境界 ---
    print_info(&42);
    print_info(&"hello");
    print_info(&3.14);

    // --- 複数のトレイト境界 ---
    print_debug_and_display(&42);
    print_debug_and_display(&"hello");

    // --- where句 ---
    let result = process(&"hello", &42);
    println!("{result}");

    // --- Display と Debug ---
    let temp = Temperature { celsius: 36.5 };
    println!("Display: {temp}");
    println!("Debug: {temp:?}");

    // --- Clone と Copy ---
    let original = Config {
        name: String::from("timeout"),
        value: 30,
    };
    let cloned = original.clone();
    println!("元: {:?}", original);
    println!("複製: {:?}", cloned);

    let a: i32 = 42;
    let b = a;
    println!("a={a}, b={b}");

    // --- PartialEq と PartialOrd ---
    let sa = Score { value: 85 };
    let sb = Score { value: 92 };
    let sc = Score { value: 85 };

    println!("a == c: {}", sa == sc);
    println!("a != b: {}", sa != sb);
    println!("a < b: {}", sa < sb);
    println!("b > c: {}", sb > sc);

    // --- トレイトオブジェクト ---
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("ポチ"),
        }),
        Box::new(Cat {
            name: String::from("タマ"),
        }),
        Box::new(Dog {
            name: String::from("ハチ"),
        }),
    ];

    for animal in &animals {
        println!("{}: {}", animal.name(), animal.speak());
    }
}
