# スクラップビューアー仕様

## 概要

調査日: 2026-10-02。本文は現在のコードを案内する資料であり、実ファイルに優先しない。

ローカルの Markdown スクラップを扱うデスクトップアプリの開発リポジトリ。現在は Tauri 2、Svelte 5、TypeScript、Vite、Tailwind CSS 4、daisyUI 5 による最小構成で、日本語の入力を Rust に送り、応答または入力エラーを画面に表示する。

Markdown 閲覧、文書収集、検索、タグ編集、設定ファイルの読み込み、Git 同期は設計資料に記載されているが、現在のアプリには実装されていない。SvelteKit やルートの Cargo workspace は使用していない。

## フォルダと主要ファイルの役割

- `/`: フロントエンドの依存・ビルド設定と開発手順。
  - `README.md`: Windows 開発環境の確認記録と起動手順。「Tauri構成が未導入」という記述は現在のコードと一致していない。
  - `package.json`: pnpm で使用する依存関係と、開発・ビルド・型チェック・Tauri CLI のスクリプト。
  - `pnpm-lock.yaml`: フロントエンドの依存解決を固定する。主な解決版は Tauri API/CLI 2.12.1、Svelte 5.57.1、Vite 8.3.1、TypeScript 6.0.3、Tailwind CSS 4.3.3、daisyUI 5.7.47。
  - `index.html`: 日本語の HTML 文書、`#app` のマウント先、`src/main.ts` の読み込み。
  - `vite.config.js`: Svelte・Tailwind CSS プラグイン、静的ファイルの配置先、開発サーバーを設定する。
  - `svelte.config.js`: `vitePreprocess` による前処理設定。
  - `tsconfig.json`: フロントエンドの strict 型チェック、ES2022、bundler 向けモジュール解決を設定する。
  - `mise.toml`: Nushell を使う `setup`、`dev`、`check`、`build` の開発タスク。
  - `.gitignore`: 依存・生成物、環境変数ファイル、Vite の一時設定ファイルを Git 管理から外す。
- `src/`: Svelte と TypeScript の画面実装。
  - `src/main.ts`: `#app` を確認し、Svelte の `mount` で `App` を起動する。
  - `src/App.svelte`: 入力フォーム、送信状態、Rust コマンド呼び出し、成功・失敗の結果表示をまとめる。
  - `src/app.css`: Tailwind CSS を読み込み、daisyUI を登録する。
- `static/`: Vite が公開する静的画像。
  - `static/favicon.png`: `index.html` が参照するファビコン。
  - `static/svelte.svg`、`static/tauri.svg`、`static/vite.svg`: ロゴ画像。現在の `App.svelte` からの参照はない。
- `src-tauri/`: Rust アプリと Tauri の構成。
  - `src-tauri/Cargo.toml`: Rust 2021 edition のパッケージ、`scrap_viewer_lib`、Tauri・serde 系の依存、リリース最適化を定義する。
  - `src-tauri/Cargo.lock`: Rust の依存解決を固定する。Tauri は 2.12.1、tauri-build は 2.7.1。
  - `src-tauri/build.rs`: `tauri_build::build()` を実行するビルドスクリプト。
  - `src-tauri/tauri.conf.json`: 開発・ビルドコマンド、フロントエンドの参照先、800×600 のウィンドウ、CSP、バンドルとアイコンを設定する。
  - `src-tauri/.gitignore`: Rust のビルド生成物と Tauri の生成スキーマを Git 管理から外す。
- `src-tauri/src/`: Rust の起動処理と画面から呼ぶコマンド。
  - `src-tauri/src/main.rs`: `scrap_viewer_lib::run()` を呼ぶ実行ファイルの入口。Windows のリリース版ではコンソールウィンドウを抑制する。
  - `src-tauri/src/lib.rs`: `greet` コマンドの入力検証・応答と、Tauri のコマンド登録・起動を実装する。
- `src-tauri/capabilities/`: Tauri の権限設定。
  - `src-tauri/capabilities/default.json`: `main` ウィンドウへ `core:default` 権限を割り当てる。
- `src-tauri/icons/`: Tauri のバンドル用アイコン画像。使用する画像は `tauri.conf.json` が指定する。
- `.plans/`: 設計案、実装計画、リポジトリ調査の設定と結果。
  - `.plans/spec-config.yaml`: 調査の除外対象と固定説明を指定する。
  - `.plans/SPEC.md`: 現在の構成と実装の案内図。
  - `.plans/20261002-1408-スクラップビューアー全体設計案.md`: Tauri 移行後の機能・処理分担・実装順序の設計案。
  - `.plans/20261002-1429-Tauri最小構成.md`: 最小構成の導入手順と実装結果。実機での動作確認結果の記録は残件とされている。
- `.plans/旧実装プラン/`: Tauri 移行前の設計と実装計画。
  - `.plans/旧実装プラン/20261002-1108-スクラップビューアー全体設計案.md`: 旧方式の全体設計案。
  - `.plans/旧実装プラン/20261002-1115-スクラップビューアー段階別実装計画.md`: egui 系 GUI を候補とした段階別計画。
  - `.plans/旧実装プラン/20261002-1128-スクラップビューアー段階1～3の詳細計画.md`: 旧方式の GUI・閲覧操作・設定読み込みの検証計画。
- `.scraps/`: 技術選定の参考記録。
  - `.scraps/20261002-1402-スクラップビューアーのTauri移行とUI選定.md`: 旧表示方式の操作上の問題と、Tauri・Svelte・daisyUI を選んだ経緯。
- `output/`: 出力フォルダ

## 実装のつながり

1. Tauri CLI は開発時に `pnpm run dev` を起動し、`http://localhost:1420` を表示する。Vite は `127.0.0.1:1420` に固定され、ポート使用中には別ポートへ移らない。Rust 配下は Vite の監視対象から外す。
2. `index.html` → `src/main.ts` → `src/App.svelte` の順に画面を起動する。`App.svelte` が `app.css` を読み込み、Vite の Tailwind CSS プラグインと daisyUI がスタイルを処理する。
3. フォーム送信は `confirmInput` が受け取り、ページ遷移を抑制する。送信中フラグとボタンの無効化で重複送信を防ぎ、`invoke<string>("greet", { name })` で Rust に入力を渡す。
4. `src-tauri/src/main.rs` → `run()` で起動した Tauri は、`generate_handler![greet]` によりコマンドを登録する。`greet` は前後の空白を除き、空なら `Err("確認する文字を入力してください。")`、それ以外なら `Ok("Rustで受け取りました：…")` を返す。
5. 画面は成功時に戻り値を表示する。失敗時は文字列エラーをそのまま表示し、それ以外には日本語の通信失敗案内を出す。`finally` で送信中状態を解除する。
6. リリースビルド前に Tauri が `pnpm run build` を実行し、フロントエンド成果物を `frontendDist` で参照する。`mise` の `build` は `pnpm exec tauri build --no-bundle` を使う。

権限は `core:default`、独自コマンドは `greet` のみ。CSP はローカル画面と IPC を許可し、開発用設定では Vite の WebSocket とインラインスタイルを追加で許可する。現在のコードにファイル操作・シェル操作・外部通信処理はない。

## 既存の実装パターン

- 画面は単一の Svelte コンポーネントにまとまり、`$state` で入力・結果・送信中状態を管理する。フォームには `onsubmit`、入力には `bind:value` を使う。
- UI は Tailwind CSS のユーティリティと daisyUI の `input`・`btn` を組み合わせる。入力にラベルを付け、結果は `aria-live="polite"` で通知する。Rust の応答は通常の文字列補間で表示する。
- TypeScript のイベントと `invoke` の戻り値に型を付ける。画面処理名は `confirmInput`、Rust の公開コマンド名は `greet` で、双方のコマンド名・引数名を一致させる。
- Rust コマンドは `#[tauri::command]` と `Result<String, String>` を使い、入力検証を Rust 側で行う。アプリ起動失敗は `expect`、フロントエンドのマウント先不足は `throw new Error` で停止する。
- Rust の実行入口と Tauri 初期化を分ける。現状では追加のサービス層・共有モジュール・独自のデータ型やスキーマは定義されていない。
- pnpm と Cargo のロックファイルを保管する。開発タスクの `check` は `pnpm run check` と `cargo check --manifest-path src-tauri/Cargo.toml` を実行する。調査対象内に自動テストファイルやテスト用スクリプトは確認できず、実機確認の項目は最小構成の計画に記録されている。
