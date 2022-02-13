# Tour of Rust

<p align="left">
    <!-- Rust icon -->
    <img src="https://img.shields.io/badge/-Rust-important?logo=Rust" />
    <!-- code size -->
    <img src="https://img.shields.io/github/languages/code-size/kei-academic/Tour-of-Rust" />
</p>

## 環境
- macOS Monterey (version 12.1)
- チップ: Apple M1
- メモリ: 8GB
- シェル: zsh
- IDE: VSCode

## インストール
基本的には[このページ](https://www.rust-lang.org/ja/tools/install)に従う．
1. 下記コマンドをターミナル上で実行．

    ```sh
    ❯ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. 下の画面表示後に`1`を入力しEnter．

    ```sh
    1) Proceed with installation (default)
    2) Customize installation
    3) Cancel installation
    ```

3. 下記コマンドをターミナル上で実行．

    ```sh
    ❯ source $HOME/.cargo/env
    ```

4. 下記コマンドをそれぞれターミナル上で実行し，versionが表示されていればインストール完了．
    - __rustup__... Rustのインストールコマンド．
    - __rustc__... Rustのコンパイラ．
    - __cargo__... パケットマネージャー．

    ```
    ❯ rustup --version
    rustup 1.24.3 (ce5817a94 2021-05-31)
    info: This is the version for the rustup toolchain manager, not the rustc compiler.
    info: The currently active `rustc` version is `rustc 1.58.1 (db9d1b20b 2022-01-20)`

    ❯ rustc --version
    rustc 1.58.1 (db9d1b20b 2022-01-20)

    ❯ cargo --version
    cargo 1.58.0 (f01b232bc 2022-01-19)
    ```

## VSCodeの設定
- Code Runner
    - `Run Code`ボタンを押すだけで，コンパイルと実行を自動で行ってくれる．
- Rust
    - 公式のExtension．
- rust-analyzer
    - 公式のExtension．コーディング中の構文チェックが有効になる．
    - 静的検査に必要なツールを`rustup`コマンドを使ってインストールし rust-analyzer を有効化する必要があるので，ターミナル上で下記コマンドを実行．
        ```sh
        ❯ rustup component add rust-analysis
        ❯ rustup component add rls
        ❯ rustup component add rust-src
        ```

## Documents
- [The Rust Programming Language](https://doc.rust-lang.org/book/)<br>... 公式ドキュメント．
- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/index.html)<br>... 日本語版公式ドキュメント．

## Learning
- [Tour of Rust](https://tourofrust.com/00_ja.html)<br>... Rustのチュートリアル．
- [Playground](https://play.rust-lang.org/)<br>... 実行ボタンのオプションを選択するとアセンブリなどまで吐き出してくれる．
- [Rustの最初のステップ](https://docs.microsoft.com/ja-JP/learn/paths/rust-first-steps/)<br>... Microsoft社が提供する Rust の入門講座．
- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/)<br>... サンプルスクリプトを通じて Rust を学んでいける資料．
- [とほほのRust入門](https://www.tohoho-web.com/ex/rust.html)<br>... サクサク Rust の文法を見ていける．
- [RustCoder ―― AtCoder と Rust で始める競技プログラミング入門](https://zenn.dev/toga/books/rust-atcoder)<br>... 競技プログラミングを Rust でやりたい方向けに書かれた資料．

## References
- [プログラミング言語Rustのススメ](https://qiita.com/elipmoc101/items/3c8b6d8332a9019e578c)<br>... Rustの概要を説明しているQiitaの記事．
- [Rust入門](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer)<br>... 公式ドキュメントを噛み砕いて説明しているZennの記事．
- [Rust を始めるための資料集](https://blog-dry.com/entry/2021/01/23/141936)<br>... Rust をはじめるための資料をリストアップしている記事．
