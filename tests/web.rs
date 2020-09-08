//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate chip_8_wasm;
use chip_8_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);


#[wasm_bindgen_test]
fn load_rom() {
    let mut cs = machine::Console::new();
    cs.load_rom("PONG");
    assert!(true);
}
