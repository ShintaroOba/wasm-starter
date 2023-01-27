- [wasm-packコマンドについて](#wasm-packコマンドについて)
- [wasmのコード生成](#wasmのコード生成)
  - [Cargo.toml](#cargotoml)
    - [1. crate-type](#1-crate-type)
  - [lib.rs](#librs)
  - [1. wasm\_bindgen](#1-wasm_bindgen)
- [Bundlerなしで動かす](#bundlerなしで動かす)
- [Denoで動かす](#denoで動かす)
  - [Install](#install)


# wasm-packコマンドについて
- ``new``: wasm-packのテンプレートプロジェクト生成
    ```
    $ wasm-pack new wasm-starter
    ```
- ``build``: `pkg`ディレクトリにwasmファイルとJsファイルが生成される
    ```
    $ wasm-pack build
    $ tree pkg /f
    
    ├─pkg
    │─ .gitignore
    │─ README.md
    │─ wasm_starter.d.ts
    │─ wasm_starter.js
    │─ wasm_starter_bg.js
    │─ wasm_starter_bg.wasm
    │─ wasm_starter_bg.wasm.d.ts
    ```
- ``pack``/ ``publish``: tarballを生成、npmレジストリへのpublish
  - 割愛。[参考](https://rustwasm.github.io/docs/wasm-pack/commands/pack-and-publish.html)

# wasmのコード生成
`wasm-pack new wasm-starter`によって自動作成されたファイルの中身と、wasmビルドした際に生成されたコードの中身を確認する。

## Cargo.toml
```toml

[lib] 
crate-type = ["cdylib", "rlib"]   # ---※1

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2.63"
console_error_panic_hook = { version = "0.1.6", optional = true }

wee_alloc = { version = "0.4.5", optional = true }

[dev-dependencies]
wasm-bindgen-test = "0.3.13"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"

```
### 1. crate-type
Rustコンパイラがクレートをどのように（静的/動的）リンクするかを定義するもの。（詳細は[Linkage](https://doc.rust-lang.org/reference/linkage.html)を参照）


- `cdylib`というのは動的ライブラリの意。ビルド時に多言語から動的にロードされる事を指す。
- `rlib`は`"R"ust "Lib"rary`の略。静的なRustライブラリであることを指しており、中間生成物として使用される。

## lib.rs
```rs
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]   # ------※1
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

```

## 1. wasm_bindgen
wasm_bindgenはWebAssemblyエコシステムの一部であり、wasmモジュールとJavascript間をブリッジする役割を担う。
`#[wasm_bindgen]`が付与された関数が外部に公開される。


# Bundlerなしで動かす
https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

# Denoで動かす
Wasmランタイムは複数あるが、Denoを使ってWasmモジュールを動かしてみる。

## Install 
```bash
$ choco install deno
```

```bash
$ deno -V
deno 1.30.0
```

