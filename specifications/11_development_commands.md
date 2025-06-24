# **CMD-1. 開発コマンド規定 (Development Command Standards)** {#CMD-1}

## **CMD-1.1. 概要** {#CMD-1.1}

本プロジェクトにおける開発時に使用するコマンドの規定と標準化されたワークフローを定義します。

## **CMD-1.2. パッケージマネージャー規定** {#CMD-1.2}

### **CMD-1.2.1. 使用必須ツール** {#CMD-1.2.1}

- **Bun**: JavaScriptランタイム・パッケージマネージャーとして使用
- **npm禁止**: `npm`コマンドの使用は禁止

### **CMD-1.2.2. 理由** {#CMD-1.2.2}

- パフォーマンス向上
- 依存関係管理の一貫性
- プロジェクト環境の統一

## **CMD-1.3. 開発コマンド一覧** {#CMD-1.3}

### **CMD-1.3.1. 基本開発コマンド** {#CMD-1.3.1}

#### **開発サーバー起動**
```bash
bun --bun run tauri dev
```
- **用途**: 開発時の動作テスト・デバッグ
- **説明**: ホットリロード機能付きでアプリケーションを起動

#### **依存関係のインストール**
```bash
bun install
```

#### **ビルド（本番用）**
```bash
bun --bun run tauri build
```

#### **ビルド（デバッグ用）**
```bash
bun --bun run tauri build -- --debug
```

### **CMD-1.3.2. Rustバックエンド関連** {#CMD-1.3.2}

#### **コンパイルチェック**
```bash
cd src-tauri
cargo check
```

#### **テスト実行**
```bash
cd src-tauri
cargo test
```

#### **マイグレーション実行**
```bash
cd src-tauri
sea-orm-cli migrate up --database-url sqlite://mind_track.sqlite
```

#### **Entity再生成**
```bash
cd src-tauri
sea-orm-cli generate entity \
    --database-url sqlite://mind_track.sqlite \
    --output-dir src/entity \
    --with-serde both \
    --date-time-crate chrono
```

### **CMD-1.3.3. フロントエンド関連** {#CMD-1.3.3}

#### **型チェック**
```bash
bun run check
```

#### **Linting**
```bash
bun run lint
```

#### **フォーマット**
```bash
bun run format
```

## **CMD-1.4. 禁止事項** {#CMD-1.4}

### **CMD-1.4.1. 使用禁止コマンド** {#CMD-1.4.1}

```bash
# ❌ 禁止
npm install
npm run [script]
npx [command]

# ✅ 正しい
bun install
bun run [script]
bunx [command]
```

## **CMD-1.5. 開発ワークフロー** {#CMD-1.5}

### **CMD-1.5.1. 日常開発フロー** {#CMD-1.5.1}

1. **開発環境起動**
   ```bash
   bun --bun run tauri dev
   ```

2. **コード変更・実装**

3. **コンパイルチェック（必要に応じて）**
   ```bash
   cd src-tauri && cargo check
   ```

4. **動作確認**
   - 開発サーバーでリアルタイム確認

### **CMD-1.5.2. 機能追加時のフロー** {#CMD-1.5.2}

1. **データベーススキーマ変更（必要な場合）**
   ```bash
   cd src-tauri
   sea-orm-cli migrate generate [migration_name]
   sea-orm-cli migrate up --database-url sqlite://mind_track.sqlite
   ```

2. **Entity再生成**
   ```bash
   cd src-tauri
   sea-orm-cli generate entity \
       --database-url sqlite://mind_track.sqlite \
       --output-dir src/entity \
       --with-serde both \
       --date-time-crate chrono
   ```

3. **実装・テスト**
   ```bash
   bun --bun run tauri dev
   ```

## **CMD-1.6. トラブルシューティング** {#CMD-1.6}

### **CMD-1.6.1. 一般的な問題** {#CMD-1.6.1}

#### **依存関係の問題**
```bash
# キャッシュクリア
bun pm cache rm

# 再インストール
rm -rf node_modules bun.lockb
bun install
```

#### **Rustコンパイルエラー**
```bash
cd src-tauri
cargo clean
cargo check
```

### **CMD-1.6.2. データベース関連** {#CMD-1.6.2}

#### **マイグレーションリセット**
```bash
cd src-tauri
rm mind_track.sqlite
sea-orm-cli migrate up --database-url sqlite://mind_track.sqlite
```

## **CMD-1.7. 検討事項** {#CMD-1.7}

- CI/CDパイプラインでのBun使用に関する設定
- 開発環境セットアップスクリプトの作成
- プロジェクト固有のコマンドエイリアスの定義
