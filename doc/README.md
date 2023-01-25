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
  https://rustwasm.github.io/docs/wasm-pack/commands/pack-and-publish.html 参照

# 