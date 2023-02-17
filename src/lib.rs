use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{engine::general_purpose, Engine as _};

use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"GrayScale called".into());

    let _base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    let mut _img = load_from_memory(&_base64_to_vector).unwrap();
    log(&"Image loaded".into());

    _img = _img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut _buffer = vec![];
    _img.write_to(&mut _buffer, Png).unwrap();
    log(&"New image written".into());

    let _encoded_img = general_purpose::STANDARD.encode(&_buffer);
    let _data_url = format!(
        "data:image/png;base64, {}",
        _encoded_img
    );

    _data_url
}