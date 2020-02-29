mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-frames!");
}

#[wasm_bindgen]
pub fn process_image(
    source_context: &CanvasRenderingContext2d,
    target_context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
) {
    let image_data: Result<ImageData, JsValue> =
        source_context.get_image_data(0.0, 0.0, width, height);
    let data: Clamped<Vec<u8>> = image_data.unwrap().data();

    let msg = format!(
        "proces_image got {} x {}; {}/{}",
        width,
        height,
        data.len(),
        data.capacity()
    );

    let output = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data.clone()), 640, 480);
    let _put_result = target_context.put_image_data(&output.unwrap(), 0.0, 0.0);
}
