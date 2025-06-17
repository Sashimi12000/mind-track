# Mindtrack

Mindtrackは、ユーザーが日々の心身の状態を客観的に把握し、「超スモールステップ」でのタスク実行を習慣化することで、無理なく自己規律を段階的に構築することを支援するアプリケーションです。

## 🎯 目的

本ツール「Mindtrack」は、ユーザーが日々の心身の状態を客観的に把握し、「超スモールステップ」（心理的抵抗を最小限にする極めて小さな行動単位）でのタスク実行を習慣化することで、無理なく自己規律を段階的に構築することを支援します。ワーキングメモリへの負荷を徹底的に軽減し、ユーザーが「できた！」という小さな成功体験を日々積み重ねられるよう設計することで、自己効力感を育み、持続的な行動変容を促します。
本ツールはWindows, macOS, iOSデバイスで利用可能とし、ユーザーが日常的に利用する環境で、自己内省と行動計画・実行を一貫してサポートすることを目指します。

## ✨ 主要機能

*   **デイリーチェックイン機能**: 毎日の心身の状態、気分、そして「やるべきこと」に対する感情を記録し、自己認識を深める。
*   **マイクロタスクプランナー機能**: 「超スモールステップ」の具体的な行動タスクを設定し、実行をサポートする。
*   **達成ログ＆承認フィードバック機能**: 小さな行動の完了を記録・可視化し、ポジティブなフィードバックを通じて自己肯定感を高める。
*   **ふりかえり機能**: 過去の記録（気分、体調、タスク達成状況）を振り返り、自身のパターンや進捗を客観的に確認する。
*   **リマインダー機能**: デイリーチェックインやタスク実行を適切なタイミングで通知し、習慣化をサポートする。

## 🛠️ 技術スタック

*   **デスクトップ/モバイルアプリケーションフレームワーク**: Tauri 2.0 (対応プラットフォーム: Windows, macOS, iOS)
*   **フロントエンド**: SvelteKit (HTML, TypeScript), Tailwind CSS, daisyUI
*   **バックエンド**: Rust
*   **データベース**: SQLite (ローカル)
*   **パッケージマネージャー**: bun

## 🚀 開発セットアップ

### 前提条件

*   [Rust](https://www.rust-lang.org/tools/install)
*   [Node.js](https://nodejs.org/) (LTS推奨)
*   [Bun](https://bun.sh/docs/installation)
*   Tauri CLI: `cargo install tauri-cli`
*   SeaORM CLI: `cargo install sea-orm-cli`

### インストール

1.  リポジトリをクローンします:
    ```bash
    git clone <repository-url>
    cd mind-track
    ```
2.  フロントエンドの依存関係をインストールします:
    ```bash
    bun install
    ```
3.  Rustの依存関係はビルド時に自動的に解決されます。

### データベースマイグレーション

1.  マイグレーション用の `.env` ファイルを `src-tauri/migration/` ディレクトリに作成します。
    ```
    DATABASE_URL="sqlite:../../mind_track.sqlite?mode=rwc"
    ```
    注意: `DATABASE_URL` のパスはプロジェクトルートからの相対パスです。`mind_track.sqlite` ファイルはプロジェクトのルートディレクトリに作成されます。

2.  マイグレーションを実行します:
    ```bash
    cd src-tauri/migration
    sea-orm-cli migrate up
    cd ../..
    ```
    または、プロジェクトルートから以下のように実行することも可能です（`DATABASE_URL` を直接指定する場合）:
    ```bash
    sea-orm-cli migrate up --database-url "sqlite:./mind_track.sqlite?mode=rwc" --migration-dir ./src-tauri/migration/
    ```

### エンティティ生成 (必要な場合)

マイグレーション後、データベーススキーマからSeaORMエンティティを生成します。

```bash
sea-orm-cli generate entity -u "sqlite:./mind_track.sqlite?mode=rwc" -o ./src-tauri/src/entity --with-serde both
```
これにより、`src-tauri/src/entity` ディレクトリにエンティティファイルが生成されます。`mod.rs` も適宜更新してください。

### 開発サーバー起動

```bash
bun run tauri dev
```

### ビルド

```bash
bun run tauri build
```

## 📝 ライセンス

このプロジェクトは MIT License の下でライセンスされています。詳細は [`LICENSE.md`](LICENSE.md) ファイルを参照してください。
