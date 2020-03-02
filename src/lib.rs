mod utils;

//use packed_simd::u8x64;
use std::cmp::{max, min};
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

#[derive(Debug)]
struct ImageMatrix {
    width: usize,
    height: usize,
    data: Box<[u8]>,
}

impl std::ops::Index<(usize, usize)> for ImageMatrix {
    type Output = u8;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let x = min(max(0, index.0), self.width - 1);
        let y = min(max(0, index.1), self.height - 1);

        &self.data[self.width * y + x]
    }
}

const IMG_WIDTH: usize = 640;
pub fn padded_buffer(buffer: &[u8; IMG_WIDTH]) -> [u8; 642] {
    let mut padded_buffer = [0u8; IMG_WIDTH + 2];
    padded_buffer[1..].copy_from_slice(buffer);
    return padded_buffer;
}

#[wasm_bindgen]
pub fn process_image(
    source_context: &CanvasRenderingContext2d,
    target_context: &CanvasRenderingContext2d,
    width: usize,
    height: usize,
) {
    utils::set_panic_hook();

    let image_data: Result<ImageData, JsValue> =
        source_context.get_image_data(0.0, 0.0, width as f64, height as f64);

    let input_pix = image_data.unwrap().data();

    let intensity = ImageMatrix {
        width: 640,
        height: 480,
        data: input_pix
            .chunks_exact(4)
            .map(|chunk| -> u8 { Iterator::sum(chunk[..2].iter().map(|i| i / 3)) })
            .collect(),
    };

    let g_y = (0..intensity.data.len()).map(|idx| {
        let x = idx % intensity.width;
        let y = idx / intensity.width;

        return (intensity[(x - 1, y - 1)] / 8)
            + (intensity[(x, y - 1)] / 4)
            + (intensity[(x + 1, y - 1)] / 8)
            - (intensity[(x - 1, y + 1)] / 8)
            - (intensity[(x, y + 1)] / 4)
            - (intensity[(x + 1, y + 1)] / 8);
    });

    let g_x = (0..intensity.data.len()).map(|idx| {
        let x = idx % intensity.width;
        let y = idx / intensity.width;

        return (intensity[(x - 1, y - 1)] / 8)
            + (intensity[(x - 1, y)] / 4)
            + (intensity[(x - 1, y + 1)] / 8)
            - (intensity[(x + 1, y - 1)] / 8)
            - (intensity[(x + 1, y)] / 4)
            - (intensity[(x + 1, y + 1)] / 8);
    });

    let sobel_norm: Box<[u8]> = g_x
        .zip(g_y)
        .map(|(x, y)| ((x * x + y * y) as f64).sqrt() as u8)
        .map(|v| (255u8 - v) * 10) // something's wrong, but this looks kinda cool
        .collect();

    let mut output_rgba: Vec<u8> = sobel_norm
        .as_ref()
        .iter()
        .map(|i| vec![*i, *i, *i, 255])
        .flatten()
        .collect();

    let output_rgba_s = output_rgba.as_mut_slice();

    let output = ImageData::new_with_u8_clamped_array_and_sh(Clamped(output_rgba_s), 640, 480);
    let _put_result = target_context.put_image_data(&output.unwrap(), 0.0, 0.0);
}
