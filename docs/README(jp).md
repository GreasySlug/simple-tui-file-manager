# simple-tui-file-manager

tui-rsを練習用に使っているシンプルなファイルマネージャー

## キーマップ

|キー     | 説明                  |
| ------- | --------------------- |
| j, up   | 上へ移動              |
| k, down | 下へ移動              |
| h, Left | 親ディレクトリへ移動  |
| l, Right| 子ディレクトリへ移動 |
| tab     | 次のディレクトリタブへ|
| S+tab   | 前のディレクトリタブへ|
| q       | すぐに終了            |

## インストール

cargo buildのみ

```sh
git clone https://github.com/GreasySlug/simple-tui-file-manager.git
rustup update
cd simple-tui-file-manager
cargo build
```

## コントリビュートするために

これはGithubとRustを練習するためのプロジェクトです

この[ドキュメント](../contribute(jp).md)も参照してください
