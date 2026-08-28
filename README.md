# 実行方法

## 1. Rustのインストール(Ubuntu / Linux)

WSLなどを使ってLinuxで実行するのがオススメです。

ターミナルで以下を実行。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

インストール後、ターミナルを開き直す。

インストールできたことを確認。

```bash
rustc --version
```

以下のようにバージョンが表示されればOK。

```text
rustc 1.xx.x
```

## 2. プロジェクトをクローン

```bash
git clone https://github.com/DanielOkada/image-processing.git
```

```bash
cd image-processing
```

## 3. 実行する

コードは`src/bin`にある。

以下のコマンドで実行。
```bash
cargo run --bin <ファイル名>
```

例えば、`canny.rs`を実行するとき
```bash
cargo run --bin canny
```

## 4. 結果確認

このような出力が出れば成功。
```
width: 1003, height: 1003
```

`out`に処理した画像が出力される。