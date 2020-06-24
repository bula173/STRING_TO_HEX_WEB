mod utils;
extern crate hex;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
//pub fn instantiate_module(module: &Module, imports: &Object) -> Promise

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn to_hex(string :&str, del :&str) -> String {

        let mut ret  = String::new();
        for c in string.chars() {
            ret.push_str(del);
            ret.push_str(&hex::encode_upper(c.to_string()));
        }
        return ret;
}
