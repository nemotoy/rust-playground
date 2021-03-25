# Troubleshooting

## 複数プロジェクトを作成した場合に補完が効かない

リポジトリの root に *Cargo.toml* を使いワークスペースを作成し、 *members* に各プロジェクト名を指定する。(※ あくまでテスト用プロジェクト)

### ref

- [Cargoのワークスペース - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html)
- [[rust-analyzer]VScodeでRustの補完機能を使いたいのに動かない](https://zenn.dev/fah_72946_engr/articles/cf53487d3cc5fc)
