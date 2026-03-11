// Rust言語の教科書 — 第8章 サンプルコード
use std::fmt;

// --- 構造体 ---
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

struct Point {
    x: f64,
    y: f64,
}

fn create_point(x: f64, y: f64) -> Point {
    Point { x, y }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Config {
    width: u32,
    height: u32,
    title: String,
}

struct Color(u8, u8, u8);
struct Meters(f64);

// --- Rectangleとメソッド ---
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

// --- Counter ---
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn value(&self) -> u32 {
        self.count
    }
}

// --- Message (selfメソッド) ---
struct Message {
    content: String,
}

impl Message {
    fn into_uppercase(self) -> String {
        self.content.to_uppercase()
    }
}

// --- 列挙型 ---
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(dir: &Direction) {
    match dir {
        Direction::Up => println!("上に移動"),
        Direction::Down => println!("下に移動"),
        Direction::Left => println!("左に移動"),
        Direction::Right => println!("右に移動"),
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => {
            std::f64::consts::PI * r * r
        }
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    Quit,
}

fn handle_event(event: &Event) {
    match event {
        Event::Click { x, y } => {
            println!("クリック: ({x}, {y})");
        }
        Event::KeyPress(c) => {
            println!("キー入力: {c}");
        }
        Event::Quit => println!("終了"),
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration_seconds(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }

    fn is_stop(&self) -> bool {
        matches!(self, TrafficLight::Red)
    }
}

// --- Option ---
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

// --- Debug derive ---
#[derive(Debug)]
struct DebugUser {
    name: String,
    age: u32,
}

fn main() {
    // --- 構造体の定義 ---
    let user = User {
        name: String::from("田中太郎"),
        email: String::from("tanaka@example.com"),
        age: 30,
        active: true,
    };
    println!("名前: {}, 年齢: {}", user.name, user.age);

    // --- ミュータブルな構造体 ---
    let mut user = User {
        name: String::from("田中太郎"),
        email: String::from("old@example.com"),
        age: 30,
        active: true,
    };
    user.email = String::from("new@example.com");
    user.age = 31;
    println!("{}: {}", user.name, user.email);

    // --- フィールド初期化の省略記法 ---
    let p = create_point(3.0, 4.0);
    println!("({}, {})", p.x, p.y);

    // --- 構造体更新構文 ---
    let default = Config {
        width: 800,
        height: 600,
        title: String::from("My App"),
    };
    let custom = Config {
        width: 1024,
        ..default
    };
    println!("{}x{}: {}", custom.width, custom.height, custom.title);

    // --- タプル構造体 ---
    let red = Color(255, 0, 0);
    let distance = Meters(42.195);
    println!("赤: ({}, {}, {})", red.0, red.1, red.2);
    println!("距離: {}m", distance.0);

    // --- メソッド ---
    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("面積: {}", rect.area());
    println!("周囲長: {}", rect.perimeter());

    // --- Counter ---
    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("カウント: {}", counter.value());

    // --- selfメソッド ---
    let msg = Message {
        content: String::from("hello"),
    };
    let upper = msg.into_uppercase();
    println!("{upper}");

    // --- 列挙型 ---
    let dir = Direction::Up;
    move_player(&dir);

    // --- データを持つバリアント ---
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    for shape in &shapes {
        println!("面積: {:.2}", area(shape));
    }

    // --- 構造体バリアント ---
    let events = vec![
        Event::Click { x: 100, y: 200 },
        Event::KeyPress('a'),
        Event::Quit,
    ];
    for event in &events {
        handle_event(event);
    }

    // --- enumにメソッド ---
    let light = TrafficLight::Red;
    println!(
        "赤信号: {}秒, 停止? {}",
        light.duration_seconds(),
        light.is_stop()
    );

    // --- Option型 ---
    let nums = [1, 3, 4, 7, 8];
    match find_first_even(&nums) {
        Some(n) => println!("最初の偶数: {n}"),
        None => println!("偶数なし"),
    }

    // --- Optionの便利メソッド ---
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;
    println!("{}", some_value.unwrap_or(0));
    println!("{}", no_value.unwrap_or(0));
    let doubled = some_value.map(|n| n * 2);
    println!("{doubled:?}");
    println!("値あり? {}", some_value.is_some());
    println!("値なし? {}", no_value.is_none());

    // --- if let ---
    let config_max: Option<u32> = Some(100);
    if let Some(max) = config_max {
        println!("最大値の設定: {max}");
    }
    let config_min: Option<u32> = None;
    if let Some(min) = config_min {
        println!("最小値の設定: {min}");
    } else {
        println!("最小値は未設定");
    }

    // --- Display トレイト ---
    let p = Point { x: 3.0, y: 4.0 };
    println!("座標: {p}");

    // --- Debug トレイト ---
    let user = DebugUser {
        name: String::from("Alice"),
        age: 25,
    };
    println!("{user:?}");
    println!("{user:#?}");
}
