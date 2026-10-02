# Scrap Viewer

ローカルのMarkdownスクラップを閲覧するデスクトップアプリです。
現在はGUIの操作を検証する段階です。

## ビルドの準備

Rustのツールチェーンと、次の日本語フォントを用意してください。

- フォント：Noto Sans CJK JP Regular
- ファイル名：`NotoSansCJKjp-Regular.otf`
- 公式入手先：
  https://github.com/notofonts/noto-cjk/blob/main/Sans/OTF/Japanese/NotoSansCJKjp-Regular.otf
- 配置先：プロジェクト直下の `vendor/fonts/NotoSansCJKjp-Regular.otf`

公式ページの「Download raw file」からフォントを保存し、
`vendor/fonts/` を作成して配置してください。OSへのインストールは不要です。
`vendor/` はGit管理対象外なので、各ビルド環境で準備してください。

フォントの埋め込み実装後は、このファイルが未配置だとビルドに失敗します。
ビルド済みの実行ファイルにはフォントが含まれるため、
実行時に外部フォントファイルを配置する必要はありません。

## フォントのライセンス

Noto Sans CJKはSIL Open Font License 1.1で提供されています。
利用・埋め込み・アプリと一緒の再配布が許可されています。
フォント単体の販売は禁止されています。

公式ライセンス：
https://github.com/notofonts/noto-cjk/blob/main/Sans/LICENSE

フォントを含むアプリを配布する場合は、フォントの著作権表示と
ライセンス全文を配布物に添付してください。
フォントを変更する場合は、予約名などの条件にも従ってください。

## ビルドと起動

フォント配置後、プロジェクト直下で実行します。

```powershell
cargo build
cargo run
```

将来の設定項目 `appearance.japanese_font` は、
埋め込みフォントを任意の外部フォントで上書きするために使います。
未指定の場合は埋め込みフォントを使います。
