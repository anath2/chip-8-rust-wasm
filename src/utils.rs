extern crate web_sys;


pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub fn log_str(message: &str) {
    web_sys::console::log_1(&format!("{}", message).into())
}


pub fn log_u8(message: u8) {
    web_sys::console::log_1(&format!("{:0x}", message).into())
}


pub fn log_u16(message: u16) {
    web_sys::console::log_1(&format!("{:0x}", message).into())
}


pub fn log_u8_array(message: &[u8]) {
    web_sys::console::log_1(&format!("{:?}", message).into())
}


pub fn log_u16_array(message: &[u16]) {
    web_sys::console::log_1(&format!("{:?}", message).into())
}


// #[wasm_bindgen]
// extern {
//     #[wasm_bindgen(js_namespace = window, js_name = generateRandomU8)]
//     pub fn get_random_u8() -> u8;
// }


pub fn get_random_u8() -> u8 {
    let mut rand_arry = [0u8; 128];
    let crypto = web_sys::window().unwrap().crypto().unwrap();
    crypto.get_random_values_with_u8_array(&mut rand_arry).unwrap();
    rand_arry[0]
}
