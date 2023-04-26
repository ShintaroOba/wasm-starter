mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// JavaScriptのwindow.alertモジュールをコール
// #[wasm_bindgen(module = "./hoge")]等でモジュールは指定もできる
#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


// JavaScript側で呼び出すRustの関数
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, hello-wasm!");
}
