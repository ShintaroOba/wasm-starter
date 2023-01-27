- [wasmをビルドするツールについて](#wasmをビルドするツールについて)
  - [wasm-bindgen-cli](#wasm-bindgen-cli)
  - [wasm-pack](#wasm-pack)
- [wasm-packコマンド](#wasm-packコマンド)
- [wasmのコード生成](#wasmのコード生成)
  - [Cargo.toml](#cargotoml)
    - [1. crate-type](#1-crate-type)
  - [lib.rs](#librs)
  - [1. wasm\_bindgen](#1-wasm_bindgen)
- [Bundlerなしで動かす](#bundlerなしで動かす)
- [Denoで動かす](#denoで動かす)
  - [Install](#install)

# wasmをビルドするツールについて
## wasm-bindgen-cli
wasm-bindgenで定義されたRustのコードからwasmモジュールとJsファイル、Tsの型定義ファイルを生成する。

## wasm-pack
上記に加え、npmモジュールとしてフロントエンドから呼び出すためのビルドツール



# wasm-packコマンド
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
  - このとき、Cargo.tomlに`[package.metadata.wasm-pack.profile.release] wasm-opt = false`を追記することで上記の生成物に加えpackage.jsonも生成される。
    ```json
    {
      "name": "wasm-starter",
      "collaborators": [
        "ShintaroOba <oba.shintaro@isid.co.jp>"
      ],
      "version": "0.1.0",
      "files": [
        "wasm_starter_bg.wasm",
        "wasm_starter.js",
        "wasm_starter_bg.js",
        "wasm_starter.d.ts"
      ],
      "module": "wasm_starter.js",
      "types": "wasm_starter.d.ts",
      "sideEffects": false
    }
    ```
- ``pack``: tarballを生成
    ```bash
    $ wasm-pack pack
    npm notice 
    npm notice 📦  wasm-starter@0.1.0
    npm notice === Tarball Contents ===
    npm notice 352B README.md
    npm notice 326B package.json
    npm notice 786B wasm_starter_bg.js
    npm notice 435B wasm_starter_bg.wasm
    npm notice 80B  wasm_starter.d.ts
    npm notice 85B  wasm_starter.js
    npm notice === Tarball Details ===
    npm notice name:          wasm-starter
    npm notice version:       0.1.0
    npm notice filename:      wasm-starter-0.1.0.tgz
    npm notice package size:  1.2 kB
    npm notice unpacked size: 2.1 kB
    npm notice shasum:        15c277ed3d34522b4423e58339301a45bbcad71b
    npm notice integrity:     sha512-aOvu5HiwQ82c6[...]tugNqko6/uhMA==
    npm notice total files:   6
    npm notice
    wasm-starter-0.1.0.tgz
    [INFO]: 🎒  packed up your package!
    ```
- `publish`: npmレジストリへ生成したtarballをpublish
  - 事前にLoginは必要
    ```bash
    $ wasm-pack login
    npm notice Log in on https://registry.npmjs.org/
    Username: shintaro
    Password: ***
    ```
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
`#[wasm_bindgen]`が付与された関数が外部に公開される。wa


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

