<h1>wasm-packを使ったwasmの生成</h1>
本資料では、wasm-packを使ってwasmプロジェクトを生成し、簡易的に動かすまでの手順と、中身のソースコードの解説を記載する。

<h2>TOC</h2>

- [そもそもWebAssemblyとは](#そもそもwebassemblyとは)
- [wasmが作られてから動くまで](#wasmが作られてから動くまで)
- [wasmのビルドツール](#wasmのビルドツール)
  - [1.wasm-bindgen-cli](#1wasm-bindgen-cli)
  - [2.wasm-pack](#2wasm-pack)
  - [3.Trunk](#3trunk)
- [wasm-packコマンド](#wasm-packコマンド)
- [テンプレートプロジェクトのソース](#テンプレートプロジェクトのソース)
  - [Cargo.toml](#cargotoml)
    - [1. crate-type](#1-crate-type)
    - [2. wasm-bindgen](#2-wasm-bindgen)
    - [3. wee\_alloc](#3-wee_alloc)
  - [lib.rs](#librs)
    - [1. externと`#[wasm-bindgen]`](#1-externとwasm-bindgen)
    - [(参考) #\[wasm-bindgen\]内では何が起こってるかを簡単に](#参考-wasm-bindgen内では何が起こってるかを簡単に)
- [wasm-pack buildによって生成されたコード](#wasm-pack-buildによって生成されたコード)
- [wasmを動かす](#wasmを動かす)
  - [Trunkで動かす](#trunkで動かす)
  - [npm initテンプレートを使う](#npm-initテンプレートを使う)
- [まとめ](#まとめ)

# そもそもWebAssemblyとは
ブラウザ上で動作するバイナリ―形式のアセンブリ言語。ネイティブアプリに近いパフォーマンスで動作することができる言語と言われている。JavaScriptではパフォーマンス的に難しい複雑な処理を補完するために誕生した。 現在WebAssemblyにコンパイルできる言語には、C/C++、Rustなどがある。


# wasmが作られてから動くまで
![](https://media.licdn.com/dms/image/C5612AQGoeSDUJsB9Ww/article-inline_image-shrink_1000_1488/0/1626616271256?e=1687996800&v=beta&t=d5puiFj4KY_uwuRfPicTDpAf0y5Qux1vvnfFLBY0s6Y)

# wasmのビルドツール
今回Rustでwasmモジュールの生成を行うにあたり、主要なビルドツールはざっくり以下の3つに分類される。  
wasm-packはwasm-bindgen-cliのラッパーなのでほぼ同じ。
wasm-packとTrunkでは生成されるモジュールや機能に違いがある。

## 1.wasm-bindgen-cli
wasm-bindgenで定義されたRustのコードからwasmモジュールとJsファイル、Tsの型定義ファイルを生成する。

## 2.wasm-pack
上記に加え、標準的な機能を持たせたもの。これが主流。  
今回はこれを使ってwasmモジュールを生成してみる。  

## 3.Trunk
wasm-packに対し、Trunkはwasmとjs、それらを呼び出すHTMLを生成。
また、開発用サーバも内包しており、フロントエンド実装を用意せずとも手軽にwasmを動かすことができる。

以下はwasmで作成したmarkdownエディタでTrunkを使って開発用サ―バを立てた時の例

![](https://user-images.githubusercontent.com/57422625/234603431-8af80347-f150-4a35-a45e-e6d483c30bbc.gif)


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

# テンプレートプロジェクトのソース
`wasm-pack new wasm-starter`によって自動作成されたファイルの中身と、wasmビルドした際に生成されたコードの中身を確認する。

## Cargo.toml
```toml

[lib] 
crate-type = ["cdylib", "rlib"]   # ---※1

[features]
default = ["console_error_panic_hook"]

[dependencies]
wasm-bindgen = "0.2.63"　　# ---※2
console_error_panic_hook = { version = "0.1.6", optional = true }

wee_alloc = { version = "0.4.5", optional = true } # ---※3

[dev-dependencies]
wasm-bindgen-test = "0.3.13"

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"

```
### 1. crate-type
Rustコンパイラがクレートをどのように（静的/動的）リンクするかを定義するもの。クレートがライブラリ、バイナリのどちらにコンパイルされるべきかをコンパイラに伝えている。（詳細は[Linkage](https://doc.rust-lang.org/reference/linkage.html)を参照）


- `cdylib`というのは動的ライブラリの意。ビルド時に多言語から動的にロードされるために使用される。Wasmの場合だと、メインエントリ関数がないということを示す。
- `rlib`は`"R"ust "Lib"rary`の略。ビルドの中間生成物であり、静的なRustライブラリであることを指す。

### 2. wasm-bindgen
JavaScriptとRust間をつなぐインターフェースとなるコードを生成することができるライブラリ
今回は、RustのコードからJavaScriptのAPIを呼びだすが、JavaScriptからRustのコードを呼び出すことも可能。
[crate.io](https://crates.io/crates/wasm-bindgen)

### 3. wee_alloc
`wee`というのは`Wasm-Enabled Elfin Allocator`の略。wasm用の軽量なメモリアロケータとしてこれを利用することでビルドして生成された.wasmファイルのサイズを小さくすることができる。

[crate.io](https://crates.io/crates/wee_alloc)

## lib.rs
```rs
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// JavaScript側のモジュールをコール
#[wasm_bindgen]   
extern {
    fn alert(s: &str);    # ------※1
}

// JavaScript側で呼び出すRustの関数
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}

```

### 1. externと`#[wasm-bindgen]`
`extern`が付与されたalert関数は外部の関数を呼び出したい場合に付与する。
`#[wasm_bindgen]`アトリビュートが付与されたことで、wasm-bindgenによって生成されたJavaScriptの関数を呼び出すことを示す。
`extern`が付いていない`greet()`はJavaScript側でこの関数を呼び出すことを表している。


### (参考) #[wasm-bindgen]内では何が起こってるかを簡単に
```rs

#[proc_macro_attribute]
pub fn wasm_bindgen(attr: TokenStream, input: TokenStream) -> TokenStream {
    match wasm_bindgen_macro_support::expand(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "xxx_debug_only_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(diagnostic) => (quote! { #diagnostic }).into(),
    }
}

```
- `#[proc_macro_attribute]`は`#[wasm-bindgen]`が手続き型マクロであることを示す
- `attr`は`#[wasm-bindgen()]`に渡す引数、`input`はアトリビュートがついた関数全体のトークン（関数のコードそのものをTokenStreamという形式になったもの）
- ``wasm_bindgen_macro_support::expand()``関数の評価をmatch式で行い、`input`に渡されたRustのコードを展開し、そこから生成したコードを返却している
  - JavaScript→Rust呼び出しの場合は、インターフェースとなるJsコードを生成。Jsコードがwasmを呼び出すことでRustの関数が呼び出される
  - Rust→JavaSCript呼び出しの場合は、生成されたJsコードはJsの関数にマッピングされる


# wasm-pack buildによって生成されたコード

- wasm_starter_bg.wasm
  - wasmモジュールそのもの
- wasm_starter_bg.js
  - Rustで実装した関数をラップしたjsの関数を生成する
- wasm_starter.js
  - wasmモジュールをインポートして初期化
- wasm_starter_bg.wasm.d.ts
- wasm_starter.d.ts



# wasmを動かす
## Trunkで動かす
未解決のIssueを引いてしまい苦しむ。
- https://github.com/thedodd/trunk/issues/321
- https://users.rust-lang.org/t/trunk-not-downloading-wasm-bindgen/88316

仕方ないのでフロントエンドの実装するしかない！

## npm initテンプレートを使う
wasmを呼び出すフロントエンド側のテンプレート実装を提供。
参考：https://rustwasm.github.io/wasm-pack/book/tutorials/hybrid-applications-with-webpack/getting-started.html#getting-started

````bash
$ npm init rust-webpack sample
````

````bash
$ npm start
````

# まとめ
- wasmの全容を知ろうとしてはいけない
- 