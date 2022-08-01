## login
```
cargo compete login atcoder
```

## 新規コンテスト作成
1. コマンド + P 
2. `task new contest`
3. コンテストID入力
（.vscode/tasks.jsonに構成したコンテストを作成するコマンドが実行される）

## 問題をブラウザで開く
```
cargo compete open --bin <問題>
```
--binつけないと問題全部ブラウザに開く

## テスト
```
cd <コンテストID>
cargo compete test <問題>
```

## 提出
```
cargo compete submit <問題>
```

## 参考
- https://qiita.com/okaponta_/items/7e82de5d1f78f547fe4b
- https://jp.magicode.io/shunnya0715/articles/9734ba4922894b329a756b6f3592e8ed

## エラー対応
cargo compete test 001
```
error: toolchain '1.42.0-aarch64-apple-darwin' is not installed
```
↓  
compete.tomlのtoolchainを空にしたらOK