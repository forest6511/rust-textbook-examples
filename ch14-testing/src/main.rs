// Rust言語の教科書 — 第14章 サンプルコード

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn internal_helper(x: i32) -> i32 {
    x * 2
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn greeting(name: &str) -> String {
    format!("こんにちは、{name}さん")
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("ゼロ除算は許可されていません");
    }
    a / b
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[derive(Debug, PartialEq)]
pub struct User {
    name: String,
    age: u32,
}

impl User {
    pub fn new(name: &str, age: u32) -> User {
        User {
            name: name.to_string(),
            age,
        }
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(3, 4) = {}", multiply(3, 4));
    println!("{}", greeting("田中"));
    println!("divide(10, 3) = {}", divide(10, 3));
    println!("reverse(\"hello\") = {}", reverse_string("hello"));
    println!("internal_helper(5) = {}", internal_helper(5));

    let user = User::new("Alice", 20);
    println!("{}は成人? {}", user.name, user.is_adult());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_internal_helper() {
        assert_eq!(internal_helper(5), 10);
    }

    #[test]
    fn test_assertions() {
        assert!(1 + 1 == 2);
        assert_eq!(4, 2 + 2);
        assert_ne!("hello", "world");
    }

    #[test]
    fn test_greeting() {
        let result = greeting("田中");
        assert!(
            result.contains("田中"),
            "挨拶に名前が含まれていません: {}",
            result
        );
    }

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    #[test]
    #[should_panic(expected = "ゼロ除算")]
    fn test_divide_by_zero_message() {
        divide(10, 0);
    }

    #[test]
    fn test_parse() -> Result<(), String> {
        let value: i32 = "42".parse().map_err(|e| format!("変換失敗: {e}"))?;
        assert_eq!(value, 42);
        Ok(())
    }

    #[test]
    fn test_reverse_basic() {
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(reverse_string(""), "");
    }

    #[test]
    fn test_reverse_single() {
        assert_eq!(reverse_string("a"), "a");
    }

    // --- Userのテスト ---
    fn create_test_user(age: u32) -> User {
        User::new("テストユーザー", age)
    }

    #[test]
    fn test_adult() {
        let user = create_test_user(20);
        assert!(user.is_adult());
    }

    #[test]
    fn test_minor() {
        let user = create_test_user(15);
        assert!(!user.is_adult());
    }

    #[test]
    fn test_boundary() {
        let user = create_test_user(18);
        assert!(user.is_adult());
    }
}
