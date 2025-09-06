# js-minimal-runtime-for-mf2

このレポジトリはフロントエンドカンファレンス北海道 2025 での LT で発表した「自作 JS エンジンに推しプロポーザルを実装したい！」のためのサンプル実装です。

- proposal : https://fortee.jp/frontend-conf-hokkaido-2025/proposal/b6be4744-2b56-40e0-9e69-e1df5d787068
- slides : https://speakerdeck.com/sajikix/js-minimal-runtime-for-mf2

## 概要

Intl.MessageFormat の提案されている仕様のうち、format 関数の機能を限定的に実行できる JavaScript エンジンを Rust で実装しています。

## 試し方

ビルドしたバイナリに JavaScript のコードのパスを渡して実行します。

```sh
cargo run -- path/to/your/javascript/file.js
```

実行すると、読み込んだ JS のコードと実行結果を出力します。

```
const mf = new Intl.MessageFormat("en", "Hello {$place}!");
mf.format({ place: "World" });

> Hello World!
```
