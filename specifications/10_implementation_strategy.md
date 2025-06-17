# **IS-1. 実装方針 (Implementation Strategy) 仕様詳細** {#IS-1}

## **IS-1.1. 概要** {#IS-1.1}

この章では、本仕様書で定義された機能をどのように実現するかの具体的な技術的アプローチと設計方針を定めます。

## **IS-1.2. バックエンド (Rust) 実装方針** {#IS-1.2}

### **IS-1.2.1. API設計 (Tauri Commands)** {#IS-1.2.1}

*   **基本原則**: フロントエンドからの呼び出しは、すべてTauriの`#[tauri::command]`アトリビュートを付けた非同期関数として定義
*   **命名規約**: `動詞_対象_補足` 形式（例: `create_micro_task`, `get_daily_checkin_by_date`）
*   **パラメータ**: 構造体による型安全なパラメータ受け渡し
*   **戻り値**: `Result<T, AppError>`の統一形式

#### **コマンド設計例**

```rust
#[tauri::command]
async fn record_daily_checkin(
    payload: DailyCheckinPayload,
    state: tauri::State<'_, AppState>
) -> Result<DailyCheckinResponse, AppError> {
    let service = DailyCheckinService::new(&state.db).await?;
    service.record_checkin(payload).await
}
```

### **IS-1.2.2. エラーハンドリング戦略** {#IS-1.2.2}

#### **AppError 設計**

*   **基本方針**: `thiserror`クレートを用いたアプリケーション固有エラー型の定義
*   **情報構造**: ユーザー向け・開発者向けの情報分離
*   **変換**: 外部ライブラリエラーの統一的変換

#### **anyhow 活用**

*   **使用場面**: 複数のエラー型を扱う内部処理、プロトタイピング段階
*   **利点**: `?`演算子による簡潔なエラー伝播
*   **境界**: Tauriコマンド境界での`AppError`への変換

#### **エラー情報設計**

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database Error: {details}")]
    Database {
        user_message: String,
        details: String,
        source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    },
    
    #[error("Validation Error on field '{field}': {message}")]
    Validation {
        user_message: String,
        field: String,
        message: String,
    },
    
    // その他のエラーバリアント
}
```

#### **シリアライゼーション**

*   **SerializableAppError**: フロントエンド向けの情報制御
*   **情報制限**: 機密情報の漏洩防止
*   **構造化**: エラーカテゴリ別の情報提供

### **IS-1.0.1. データベース操作戦略** {#IS-1.0.1}

#### **SeaORM 活用**

*   **採用理由**: 型安全、非同期対応、マイグレーション機能
*   **ActiveRecord**: モデルベースの操作
*   **クエリビルダー**: 複雑な検索条件の構築

#### **マイグレーション管理**

*   **自動実行**: アプリ起動時の未適用マイグレーション実行
*   **バージョン管理**: 段階的なスキーマ変更
*   **ロールバック**: 問題発生時の復旧機能

#### **接続管理**

*   **コネクションプール**: 効率的なDB接続管理
*   **トランザクション**: データ整合性の確保
*   **エラー処理**: DB接続失敗時の適切な処理

### **IS-1.0.2. 主要クレートの役割** {#IS-1.0.2}

| クレート | 用途 | 選定理由 |
|---------|------|----------|
| `thiserror` | 独自エラー型定義 | 型安全、マクロによる簡潔な定義 |
| `anyhow` | 内部エラーハンドリング | 柔軟性、プロトタイピングに適している |
| `uuid` | グローバル一意ID生成 | 標準的、将来のデータ同期対応 |
| `chrono` | 日時処理 | 豊富な機能、タイムゾーン対応 |
| `serde` | シリアライゼーション | JSON変換、Tauri連携 |
| `tokio` | 非同期ランタイム | 標準的、SeaORMとの親和性 |

## **IS-1.3. フロントエンド (SvelteKit) 実装方針** {#IS-1.3}

### **IS-1.3.1. 状態管理 (State Management)** {#IS-1.3.1}

#### **Svelteストア活用**

*   **Writable**: ユーザー入力、設定値等の可変状態
*   **Readable**: サーバーから取得した読み取り専用データ
*   **Derived**: 既存ストアから計算される派生状態
*   **Custom**: ビジネスロジックを含むカスタムストア

#### **ストア設計例**

```typescript
// stores/checkin.ts
export const dailyCheckinStore = writable<DailyCheckinData | null>(null);
export const isCheckinModalOpen = writable<boolean>(false);
export const checkinFormData = writable<DailyCheckinPayload>({
    date: '',
    moodLevel: 3,
    moodText: '',
    physicalStateTags: [],
    potentialTodos: [],
    feelingForTodos: ''
});
```

#### **永続化戦略**

*   **重要データ**: Tauriコマンドによるバックエンド保存
*   **UI状態**: sessionStorage/localStorageでの一時保存
*   **キャッシュ**: メモリ内での効率的なデータ保持

### **IS-1.0.3. コンポーネント設計原則** {#IS-1.0.3}

#### **原子設計 (Atomic Design)**

1.  **Atoms**: Button, Input, Icon等の基本要素
2.  **Molecules**: SearchBox, FormField等の複合要素
3.  **Organisms**: TaskList, Calendar等のセクション
4.  **Templates**: ページの骨格
5.  **Pages**: 完全なページ

#### **プロップス設計**

*   **型安全**: TypeScriptによる厳密な型定義
*   **最小性**: 必要最小限のプロップス
*   **拡張性**: 将来の機能拡張を考慮した設計

#### **イベント設計**

*   **カスタムイベント**: ビジネスロジックに応じたイベント
*   **バブリング**: 適切なイベント伝播
*   **型定義**: イベントペイロードの型安全性

### **IS-1.0.4. バックエンドとの通信** {#IS-1.0.4}

#### **API ラッパー設計**

```typescript
// lib/api/checkin.ts
export class CheckinAPI {
    static async recordDailyCheckin(
        payload: DailyCheckinPayload
    ): Promise<string> {
        try {
            return await invoke<string>('record_daily_checkin', { payload });
        } catch (error) {
            throw this.handleError(error);
        }
    }
    
    private static handleError(error: unknown): AppError {
        // エラー変換ロジック
    }
}
```

#### **エラーハンドリング**

*   **統一処理**: try-catchブロックによる一貫したエラー処理
*   **ユーザーフィードバック**: 適切なエラーメッセージ表示
*   **ログ**: 開発・デバッグ用のエラーログ

#### **ローディング状態**

*   **状態管理**: 非同期処理中の適切なローディング表示
*   **UX配慮**: ユーザーへの処理状況フィードバック
*   **エラー時**: 処理失敗時の状態復元

## **IS-1.1. プロジェクト構造設計** {#IS-1.1}

### **IS-1.1.1. ディレクトリ構造** {#IS-1.1.1}

```
mind-track/
├── src/                      # フロントエンド (SvelteKit)
│   ├── routes/              # ルーティング
│   ├── lib/                 # 共通ライブラリ
│   │   ├── components/      # UIコンポーネント
│   │   ├── stores/          # 状態管理
│   │   ├── api/            # バックエンドAPI
│   │   ├── utils/          # ユーティリティ
│   │   └── types/          # 型定義
│   └── app.html            # HTMLテンプレート
├── src-tauri/              # バックエンド (Rust)
│   ├── src/
│   │   ├── commands/       # Tauriコマンド
│   │   ├── models/         # データモデル
│   │   ├── services/       # ビジネスロジック
│   │   └── utils/          # ユーティリティ
│   ├── migration/          # DBマイグレーション
│   └── Cargo.toml         # Rust依存関係
├── specifications/         # 仕様書
└── docs/                  # ドキュメント
```

### **IS-1.1.2. 命名規約** {#IS-1.1.2}

#### **ファイル・ディレクトリ**

*   **kebab-case**: ファイル名、ディレクトリ名
*   **PascalCase**: コンポーネントファイル
*   **説明的**: 役割が明確に分かる名前

#### **変数・関数**

*   **camelCase**: JavaScript/TypeScript
*   **snake_case**: Rust
*   **UPPER_SNAKE_CASE**: 定数

#### **型定義**

*   **PascalCase**: インターフェース、型
*   **接尾辞**: 用途別の接尾辞（例: Payload, Response, Error）

## **IS-1.2. バージョニングと自動更新方針** {#IS-1.2}

### **IS-1.2.1. バージョニング体系** {#IS-1.2.1}

*   **セマンティックバージョニング**: vMAJOR.MINOR.PATCH
*   **MAJOR**: 破壊的変更
*   **MINOR**: 後方互換性のある機能追加
*   **PATCH**: 後方互換性のあるバグ修正

### **IS-1.2.2. ビルドとリリース自動化** {#IS-1.2.2}

#### **GitHub Actions ワークフロー**

```yaml
name: Release
on:
  push:
    tags: ['v*']
    
jobs:
  build:
    strategy:
      matrix:
        platform: [windows-latest, macos-latest]
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Build App
        run: npm run tauri build
      - name: Upload Release
        uses: softprops/action-gh-release@v1
```

#### **自動化項目**

*   **ビルド**: マルチプラットフォーム同時ビルド
*   **テスト**: 自動テスト実行
*   **署名**: コード署名の自動実行
*   **配布**: GitHub Releasesへの自動アップロード

### **IS-1.2.3. アプリ内更新機能** {#IS-1.2.3}

#### **Tauri Updater活用**

*   **プラグイン**: `@tauri-apps/plugin-updater`
*   **自動チェック**: アプリ起動時の更新確認
*   **ユーザー同意**: 更新前のユーザー確認
*   **段階的配布**: ユーザーグループ別の更新配布

#### **更新フロー**

1.  **更新チェック**: アプリ起動時の自動確認
2.  **ダウンロード**: バックグラウンドでの更新ファイル取得
3.  **確認**: ユーザーへの更新通知と同意確認
4.  **適用**: アプリ再起動での更新適用
5.  **検証**: 更新後の動作確認

## **IS-1.3. 開発プロセス** {#IS-1.3}

### **IS-1.3.1. 開発フロー** {#IS-1.3.1}

1.  **機能設計**: 仕様書ベースの詳細設計
2.  **プロトタイプ**: 基本機能の動作確認
3.  **実装**: 本格的な機能実装
4.  **テスト**: 単体・統合・E2Eテスト
5.  **レビュー**: コードレビューと品質確認
6.  **統合**: メインブランチへのマージ

### **IS-1.3.2. 品質保証** {#IS-1.3.2}

#### **自動化**

*   **フォーマット**: Prettier、rustfmtによる自動整形
*   **リント**: ESLint、Clippyによる静的解析
*   **テスト**: 継続的なテスト実行
*   **型チェック**: TypeScript、Rustの型安全性確認

#### **手動確認**

*   **コードレビュー**: 全変更のピアレビュー
*   **デザインレビュー**: UI/UXの一貫性確認
*   **機能テスト**: 実際の使用シナリオでの動作確認

## **IS-1.4. 不明点・検討事項リスト** {#IS-1.4}

### **IS-1.4.1. アーキテクチャ・設計関連** {#IS-1.4.1}
*   **レイヤー分離**: ビジネスロジックとプレゼンテーション層の適切な分離度
*   **依存関係**: 循環依存の回避と適切な依存方向の設計
*   **拡張性**: 将来機能追加時のアーキテクチャ影響範囲
*   **テスタビリティ**: テストしやすい設計の実現方法

### **IS-1.4.2. パフォーマンス・最適化関連** {#IS-1.4.2}
*   **バンドルサイズ**: フロントエンドの最適なバンドルサイズ
*   **起動時間**: アプリケーション起動時間の最適化目標
*   **メモリ使用量**: 長期間使用時のメモリ効率
*   **データベース**: 大量データ時のクエリパフォーマンス

### **IS-1.4.3. 開発・運用関連** {#IS-1.4.3}
*   **デバッグ**: 本番環境での効果的なデバッグ方法
*   **ログ**: 適切なログレベルと出力先の設計
*   **監視**: アプリケーション動作の監視指標
*   **バックアップ**: 開発環境・本番データのバックアップ戦略

### **IS-1.4.4. セキュリティ・品質関連** {#IS-1.4.4}
*   **入力検証**: フロントエンド・バックエンドでの検証分担
*   **暗号化**: 機密データの暗号化レベル
*   **監査**: セキュリティ監査の実施頻度と範囲
*   **品質指標**: コード品質の定量的評価方法

### **IS-1.4.5. チーム・プロセス関連** {#IS-1.4.5}
*   **コードレビュー**: 効率的なレビュープロセス
*   **ドキュメント**: 技術文書の更新・維持方法
*   **ナレッジ共有**: チーム内での知識共有方法
*   **技術選択**: 新技術導入時の評価・決定プロセス

### **IS-1.4.6. デプロイ・リリース関連** {#IS-1.4.6}
*   **環境管理**: 開発・ステージング・本番環境の構成
*   **リリース戦略**: 段階的リリースの実施方法
*   **ロールバック**: 問題発生時の迅速な復旧方法
*   **ユーザーサポート**: リリース後のユーザーサポート体制
