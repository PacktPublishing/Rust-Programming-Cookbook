use sha2::{Sha256, Digest};
use wasm_bindgen::prelude::*;

fn hex_digest(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input(data.as_bytes());
    let signature = hasher.result();
    signature
        .as_ref()
        .iter()
        .map(|b| format!("{:X}", b))
        .collect::<String>()
}

#[wasm_bindgen]
pub extern "C" fn digest(data: String) -> String {
    hex_digest(&data)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // This function is getting called when initializing the WASM module
    Ok(())
}


#[wasm_bindgen]
pub extern "C" fn digest_attach(data: String, elem_id: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    //let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.get_element_by_id(&elem_id).expect(&format!("Could not get element with id '{}'", elem_id));
    let signature = hex_digest(&data);
    val.set_inner_html(&signature);
    Ok(())
}
// No tests :( 