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

    let input_pix: Clamped<Vec<u8>> = image_data.unwrap().data();
    let mut output_pix: Vec<u8> = vec![255; input_pix.len()];
    for (input_chunk, output_chunk) in input_pix.chunks(4).zip(output_pix.chunks_exact_mut(4)) {
        let total: u16 = Iterator::sum(input_chunk[..2].iter().map(|i| *i as u16));
        let mean = total / 3;
        for i in output_chunk.iter_mut().take(3) {
            *i = mean as u8;
        }

        //output_chunk[0..2].clone_from_slice(&[mean as u8; 3]);
        //log(&format!(" {:?}", output_chunk));
    }

    let output = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut output_pix), 640, 480);
    let _put_result = target_context.put_image_data(&output.unwrap(), 0.0, 0.0);
}
