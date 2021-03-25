# Troubleshooting

## 複数プロジェクトを作成した場合に補完が効かない

リポジトリの root に *Cargo.toml* を使いワークスペースを作成し、 *members* に各プロジェクト名を指定する。(※ あくまでテスト用プロジェクト)

### ref

- [Cargoのワークスペース - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html)
- [[rust-analyzer]VScodeでRustの補完機能を使いたいのに動かない](https://zenn.dev/fah_72946_engr/articles/cf53487d3cc5fc)

## cargo install が失敗する

*cargo-edit* のインストールに失敗した際のメモ。

### 事象

```sh
» cargo install cargo-edit
    Updating crates.io index
error: failed to fetch `https://github.com/rust-lang/crates.io-index`

Caused by:
  failed to authenticate when downloading repository: git@github.com:rust-lang/crates.io-index

  * attempted ssh-agent authentication, but no usernames succeeded: `git`

  if the git CLI succeeds then `net.git-fetch-with-cli` may help here
  https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli

Caused by:
  no authentication available
```

### 解決方法

設定ファイルに下記項目を追加。

```toml:$HOME/.cargo/
[net]
git-fetch-with-cli = true
```

### 参考

- [Configuration - The Cargo Book](https://doc.rust-lang.org/cargo/reference/config.html#netgit-fetch-with-cli)
- [failed to authenticate when downloading repository · Issue #3381 · rust-lang/cargo](https://github.com/rust-lang/cargo/issues/3381)
- [cargoでfailed to authenticateになった時の対処法 - Qiita](https://qiita.com/ryo-yamaoka/items/c9d7c9127540e9eadfbb)
