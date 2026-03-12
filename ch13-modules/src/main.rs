// Rust言語の教科書 — 第13章 サンプルコード

// --- mod キーワード ---
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

// --- ネストしたモジュール ---
mod network {
    pub mod http {
        pub fn get(url: &str) -> String {
            format!("GET {url}")
        }
    }

    pub mod tcp {
        pub fn connect(addr: &str) -> String {
            format!("Connected to {addr}")
        }
    }
}

// --- 可視性の制御 ---
mod config {
    fn default_port() -> u16 {
        8080
    }

    pub fn server_address() -> String {
        let port = default_port();
        format!("0.0.0.0:{port}")
    }
}

// --- 構造体の可視性 ---
mod user {
    pub struct User {
        pub name: String,
        age: u32,
    }

    impl User {
        pub fn new(name: &str, age: u32) -> User {
            User {
                name: String::from(name),
                age,
            }
        }

        pub fn age(&self) -> u32 {
            self.age
        }
    }
}

// --- useによるインポート ---
mod geometry {
    pub mod shapes {
        pub fn circle_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }

        pub fn rect_area(w: f64, h: f64) -> f64 {
            w * h
        }
    }
}

use geometry::shapes;

// --- 名前の衝突とas ---
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

fn format_and_write() -> IoResult<()> {
    let msg: FmtResult = Ok(());
    println!("format: {msg:?}");
    Ok(())
}

// --- selfとsuperパス ---
mod parent {
    pub fn greet() -> String {
        String::from("親モジュールです")
    }

    pub mod child {
        pub fn call_parent() -> String {
            super::greet()
        }

        pub fn call_self() -> String {
            self::helper()
        }

        fn helper() -> String {
            String::from("子モジュールのヘルパー")
        }
    }
}

// --- pub use による再公開 ---
mod internals {
    pub fn compute() -> i32 {
        42
    }
}

mod api {
    pub use super::internals::compute;
}

fn main() {
    // --- mod キーワード ---
    let sum = math::add(3, 4);
    let product = math::multiply(3, 4);
    println!("3 + 4 = {sum}");
    println!("3 * 4 = {product}");

    // --- ファイル分割の例（分割前、単一ファイル） ---
    println!("{}", math::add(10, 3));
    println!("{}", math::subtract(10, 3));

    // --- ネストしたモジュール ---
    let response =
        network::http::get("https://example.com");
    let conn =
        network::tcp::connect("127.0.0.1:8080");
    println!("{response}");
    println!("{conn}");

    // --- 可視性の制御 ---
    println!("{}", config::server_address());

    // --- 構造体の可視性 ---
    let u = user::User::new("Alice", 30);
    println!("名前: {}", u.name);
    println!("年齢: {}", u.age());

    // --- useによるインポート ---
    let c = shapes::circle_area(5.0);
    let r = shapes::rect_area(4.0, 6.0);
    println!("円の面積: {c:.2}");
    println!("長方形の面積: {r:.1}");

    // --- 名前の衝突とas ---
    format_and_write().unwrap();

    // --- selfとsuperパス ---
    println!("{}", parent::child::call_parent());
    println!("{}", parent::child::call_self());

    // --- pub use による再公開 ---
    let result = api::compute();
    println!("結果: {result}");
}
