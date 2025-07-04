# **AR-1. アーキテクチャ概要 仕様詳細** {#AR-1}

## **AR-1.1. 全体アーキテクチャ** {#AR-1.1}

### **AR-1.1.1. レイヤー構成** {#AR-1.1.1}

```
┌─────────────────────────────────────────┐
│         プレゼンテーション層             │
│    (SvelteKit + Tailwind CSS + daisyUI) │
├─────────────────────────────────────────┤
│              Tauri Bridge               │
│         (IPC + Capabilities)            │
├─────────────────────────────────────────┤
│            アプリケーション層            │
│        (Rust - Business Logic)          │
├─────────────────────────────────────────┤
│             データアクセス層             │
│         (SeaORM + SQLite)               │
├─────────────────────────────────────────┤
│              OS ネイティブ層             │
│   (Windows/macOS/iOS System APIs)       │
└─────────────────────────────────────────┘
```

### **AR-1.1.2. 責務分離** {#AR-1.1.2}

*   **フロントエンド (SvelteKit)**: UI/UX、ユーザー入力処理、状態管理、表示ロジック
*   **バックエンド (Rust)**: ビジネスロジック、データ永続化、システム連携、セキュリティ
*   **Tauri**: フロントエンド・バックエンド間のブリッジ、OSネイティブ機能へのアクセス

## **AR-1.2. フロントエンド (SvelteKit) アーキテクチャ** {#AR-1.2}

### **AR-1.2.1. ディレクトリ構造** {#AR-1.2.1}

```
src/
├── routes/                 # ルーティング
│   ├── +layout.svelte     # 共通レイアウト
│   ├── +layout.ts         # レイアウトロード処理
│   ├── +page.svelte       # ホーム画面
│   ├── checkin/           # チェックイン関連
│   ├── tasks/             # タスク管理関連
│   ├── reflection/        # ふりかえり関連
│   └── settings/          # 設定関連
├── lib/                   # 共通ライブラリ
│   ├── components/        # 再利用可能コンポーネント
│   │   ├── forms/         # フォーム関連
│   │   ├── ui/            # UI基本コンポーネント
│   │   └── charts/        # グラフ・チャート
│   ├── stores/            # Svelteストア
│   │   ├── auth.ts        # 認証状態
│   │   ├── settings.ts    # アプリ設定
│   │   ├── checkin.ts     # チェックイン状態
│   │   └── tasks.ts       # タスク状態
│   ├── api/               # Tauri API ラッパー
│   │   ├── checkin.ts     # チェックイン API
│   │   ├── tasks.ts       # タスク API
│   │   └── settings.ts    # 設定 API
│   ├── utils/             # ユーティリティ
│   │   ├── date.ts        # 日付処理
│   │   ├── validation.ts  # バリデーション
│   │   └── constants.ts   # 定数定義
│   └── types/             # TypeScript型定義
│       ├── api.ts         # API型定義
│       ├── models.ts      # データモデル
│       └── ui.ts          # UI関連型
└── app.html               # HTMLテンプレート
```

### **AR-1.2.2. 状態管理戦略** {#AR-1.2.2}

1.  **Svelteストア**:
    *   **Writable**: 編集可能な状態（フォーム入力等）
    *   **Readable**: 読み取り専用状態（設定値等）
    *   **Derived**: 計算された状態（統計データ等）

2.  **状態の永続化**:
    *   **重要な状態**: Tauriコマンドでバックエンド保存
    *   **一時的な状態**: sessionStorage/localStorage
    *   **キャッシュ**: メモリ内での一時保存

### **AR-1.2.3. コンポーネント設計** {#AR-1.2.3}

1.  **原子コンポーネント** (Atomic):
    *   Button、Input、Modal等の基本UI
    *   daisyUIコンポーネントのラッパー

2.  **分子コンポーネント** (Molecular):
    *   SearchBox、DatePicker等の複合UI
    *   特定機能を持つコンポーネント

3.  **有機体コンポーネント** (Organism):
    *   TaskList、CalendarView等のページセクション
    *   複数の分子・原子コンポーネントで構成

4.  **ページコンポーネント** (Pages):
    *   完全なページ表示
    *   ルーティングと連動

## **AR-1.3. バックエンド (Rust) アーキテクチャ** {#AR-1.3}

### **AR-1.3.1. ディレクトリ構造** {#AR-1.3.1}

```
src-tauri/src/
├── main.rs                # エントリーポイント
├── lib.rs                 # ライブラリルート
├── app_state.rs           # アプリケーション状態
├── db.rs                  # データベース接続
├── error.rs               # エラー定義
├── commands/              # Tauri コマンド
│   ├── mod.rs
│   ├── daily_checkin.rs   # チェックイン操作
│   ├── micro_task.rs      # タスク操作
│   ├── reminder.rs        # リマインダー操作
│   └── stats.rs           # 統計操作
├── models/                # データモデル
│   ├── mod.rs
│   ├── daily_checkin.rs   # チェックインモデル
│   └── micro_task.rs      # タスクモデル
├── services/              # ビジネスロジック
│   ├── mod.rs
│   ├── daily_checkin_service.rs
│   ├── micro_task_service.rs
│   └── stats_service.rs
└── utils/                 # ユーティリティ
    ├── mod.rs
    ├── id_generator.rs    # UUID生成
    └── time_utils.rs      # 時間処理
```

### **AR-1.3.2. レイヤード アーキテクチャ** {#AR-1.3.2}

1.  **Command Layer** (commands/):
    *   Tauriコマンドの定義
    *   フロントエンドからの要求受付
    *   入力バリデーション
    *   サービス層への委譲

2.  **Service Layer** (services/):
    *   ビジネスロジックの実装
    *   複数モデルの協調処理
    *   トランザクション管理

3.  **Model Layer** (models/):
    *   データモデルの定義
    *   データベーススキーマとの対応
    *   基本的なCRUD操作

4.  **Utility Layer** (utils/):
    *   共通処理の実装
    *   外部ライブラリのラッパー

### **AR-1.3.3. エラーハンドリング** {#AR-1.3.3}

*   **AppError**: アプリケーション固有エラー型
*   **anyhow**: 内部エラー処理の簡素化
*   **エラー変換**: 外部ライブラリエラーの統一的変換

## **AR-1.4. データ層アーキテクチャ** {#AR-1.4}

### **AR-1.4.1. データベース設計** {#AR-1.4.1}

1.  **SQLite選択理由**:
    *   軽量・高性能
    *   ローカルファイル形式
    *   ACID準拠
    *   クロスプラットフォーム対応

2.  **SeaORM採用理由**:
    *   型安全なクエリ
    *   マイグレーション機能
    *   ActiveRecordパターン
    *   非同期対応

### **AR-1.4.2. テーブル設計原則** {#AR-1.4.2}

*   **正規化**: 第3正規形までの正規化
*   **UUID**: グローバル一意性の確保
*   **論理削除**: データ保全と監査証跡
*   **タイムスタンプ**: 作成・更新日時の記録

### **AR-1.4.3. インデックス戦略** {#AR-1.4.3}

*   **プライマリキー**: 自動インデックス
*   **外部キー**: 参照性能向上
*   **検索カラム**: 頻繁に検索される項目
*   **複合インデックス**: 複数条件検索最適化

## **AR-1.5. Tauri Integration** {#AR-1.5}

### **AR-1.5.1. IPC (Inter-Process Communication)** {#AR-1.5.1}

1.  **コマンド定義**:
    ```rust
    #[tauri::command]
    async fn record_daily_checkin(
        payload: DailyCheckinPayload,
        state: tauri::State<'_, AppState>
    ) -> Result<String, AppError> {
        // 実装
    }
    ```

2.  **フロントエンド呼び出し**:
    ```typescript
    import { invoke } from '@tauri-apps/api/tauri';
    
    const result = await invoke<string>('record_daily_checkin', {
        payload: checkinData
    });
    ```

### **AR-1.5.2. Capabilities 設定** {#AR-1.5.2}

*   **最小権限原則**: 必要な機能のみ許可
*   **セキュリティ**: 不要なAPIアクセス禁止
*   **プラットフォーム別**: OS固有機能の条件付き許可

### **AR-1.5.3. プラグイン活用** {#AR-1.5.3}

*   **@tauri-apps/plugin-notification**: 通知機能
*   **@tauri-apps/plugin-updater**: 自動更新
*   **@tauri-apps/plugin-store**: 設定永続化

## **AR-1.6. セキュリティアーキテクチャ** {#AR-1.6}

### **AR-1.6.1. データ保護** {#AR-1.6.1}

*   **暗号化**: 将来的なデータベース暗号化
*   **アクセス制御**: Capabilitiesによる機能制限
*   **入力検証**: フロントエンド・バックエンド双方での検証

### **AR-1.6.2. IPC セキュリティ** {#AR-1.6.2}

*   **コマンド制限**: 許可されたコマンドのみ実行
*   **データ検証**: 送受信データの整合性確認
*   **エラー情報**: 機密情報の漏洩防止

## **AR-1.7. パフォーマンス設計** {#AR-1.7}

### **AR-1.7.1. 応答性確保** {#AR-1.7.1}

*   **非同期処理**: I/O操作の非ブロッキング実行
*   **バックグラウンド処理**: 重い処理の分離
*   **キャッシュ**: 頻繁にアクセスされるデータの保持

### **AR-1.7.2. メモリ効率** {#AR-1.7.2}

*   **遅延読み込み**: 必要時データ取得
*   **ページング**: 大量データの分割処理
*   **ガベージコレクション**: 不要オブジェクトの解放

## **AR-1.8. 不明点・検討事項リスト** {#AR-1.8}

### **AR-1.8.1. アーキテクチャ設計関連** {#AR-1.8.1}
*   **マイクロサービス化**: 将来的な機能分離の可能性
*   **プラグインアーキテクチャ**: サードパーティ拡張への対応
*   **イベント駆動**: 非同期イベント処理の導入
*   **CQRS**: コマンドとクエリの責務分離
*   **DDD**: ドメイン駆動設計の適用度合い

### **AR-1.8.2. スケーラビリティ関連** {#AR-1.8.2}
*   **データ量増大**: 長期利用時のパフォーマンス維持
*   **機能拡張**: 新機能追加時のアーキテクチャ影響
*   **プラットフォーム追加**: 新OS対応時の設計変更
*   **同期機能**: 将来的なクラウド同期の設計影響

### **AR-1.8.3. 開発・保守性関連** {#AR-1.8.3}
*   **テスト戦略**: 各層でのテスト方法と自動化
*   **CI/CD**: 継続的インテグレーション・デプロイメント
*   **コード品質**: 静的解析、コードレビューの自動化
*   **ドキュメンテーション**: アーキテクチャ文書の維持

### **AR-1.8.4. 技術選択関連** {#AR-1.8.4}
*   **ライブラリ更新**: 依存関係の更新戦略
*   **技術負債**: 技術選択の長期的影響
*   **代替技術**: 現在の選択の代替案検討
*   **パフォーマンス**: ボトルネック特定と最適化

### **AR-1.8.5. セキュリティ・運用関連** {#AR-1.8.5}
*   **脆弱性対応**: セキュリティ更新の適用方法
*   **監視**: アプリケーション動作の監視方法
*   **ログ管理**: ログ出力と分析の戦略
*   **障害対応**: 障害検知と復旧の仕組み

### **AR-1.8.6. ユーザー体験関連** {#AR-1.8.6}
*   **起動時間**: アプリケーション起動の最適化
*   **メモリ使用量**: ユーザー端末への負荷軽減
*   **オフライン対応**: ネットワーク断絶時の動作
*   **データ移行**: アップデート時のユーザーデータ保護
