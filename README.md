# スクラップビューアー

## 開発環境

確認日: 2026-10-02

- Rust: 1.98.1
- Cargo: 1.98.1
- Rustツールチェーン: stable-x86_64-pc-windows-msvc
- Node.js: 24.20.0
- pnpm: 11.24.0（miseのshim経由）
- Visual Studio: C++によるデスクトップ開発を選択済み
- MSVC x64/x86ビルドツール・Windows SDK: 選択済み
- Microsoft Edge WebView2 Runtime: 154.0.4258.48

## 開発時の起動手順

Tauri構成の導入後、プロジェクトルートで実行する。

```nu
pnpm install
pnpm exec tauri dev
```
現時点ではTauri構成が未導入のため、起動は未確認。
