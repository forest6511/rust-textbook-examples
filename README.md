# Rust言語の教科書 — サンプルコード

「**Rust言語の教科書 — 基礎からコマンドラインツール開発まで**」（森川 陽介 著）のサンプルコードリポジトリです。

## 動作要件

- Rust 1.75.0 以上（`rustup update stable` で最新版に更新できます）
- Cargo（Rust に同梱）

## 使い方

各章のディレクトリに移動して `cargo run` を実行してください。

```bash
# 例: 第5章のサンプルを実行
cd ch05-ownership
cargo run

# examples/ がある章では --example で個別に実行
cd ch01-setup
cargo run --example format_specifiers
```

## 章一覧

| ディレクトリ | 章タイトル | 外部依存 |
|-------------|-----------|---------|
| `ch01-setup/` | 第1章: 環境構築とHello World | なし |
| `ch02-variables/` | 第2章: 変数・型・関数 | なし |
| `ch03-control-flow/` | 第3章: 制御フローとパターンマッチ | なし |
| `ch04-memory/` | 第4章: メモリの基礎 | なし |
| `ch05-ownership/` | 第5章: 所有権 | なし |
| `ch06-borrowing/` | 第6章: 借用とライフタイム | なし |
| `ch07-ownership-patterns/` | 第7章: 所有権の実践パターン | なし |
| `ch08-structs-enums/` | 第8章: 構造体と列挙型 | なし |
| `ch09-collections/` | 第9章: コレクションと文字列 | なし |
| `ch10-traits-generics/` | 第10章: トレイトとジェネリクス | なし |
| `ch11-error-handling/` | 第11章: エラーハンドリング | なし |
| `ch12-closures-iterators/` | 第12章: クロージャとイテレータ | なし |
| `ch13-modules/` | 第13章: モジュールとクレート | なし |
| `ch14-testing/` | 第14章: テスト | なし |
| `ch15-cli-file-tool/` | 第15章: CLIファイル検索ツール | clap, walkdir |
| `ch16-cli-json-csv-tool/` | 第16章: データ変換ツール | clap, serde, serde_json, csv |

## ライセンス

MIT License — 詳細は [LICENSE](LICENSE) を参照してください。

## 書籍について

本書は Amazon Kindle / Kindle Unlimited で販売中です。

[Rust言語の教科書 — 基礎からコマンドラインツール開発まで（Amazon）](https://www.amazon.co.jp/dp/B0GSHJ3PTQ)
