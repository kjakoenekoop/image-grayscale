extern crate core;

use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use base64::{encode as encode64, decode as decode64};
use image::{load_from_memory, ImageOutputFormat::Png};

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let base64_to_vector = decode64(encoded_file).unwrap();

    let mut buffer = Vec::new();
    load_from_memory(&base64_to_vector).unwrap()
        .grayscale()
        .write_to(&mut Cursor::new(&mut buffer), Png).unwrap();

    let encoded_image = encode64(&buffer);
    format!("data:image/png;base64,{}", encoded_image)
}
