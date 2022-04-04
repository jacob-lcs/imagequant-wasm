mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // Image loading/saving is outside scope of this library
    let width = 10;
    let height = 10;
    let fakebitmap = vec![imagequant::RGBA {r:100, g:200, b:250, a:255}; width * height];

    // Configure the library
    let mut liq = imagequant::new();
    liq.set_speed(5).unwrap();
    liq.set_quality(70, 99).unwrap();

    // Describe the bitmap
    // let mut img = liq.new_image(&fakebitmap[..], width, height, 0.0).unwrap();

    // The magic happens in quantize()
    // let mut res = match liq.quantize(&mut img) {
    //     Ok(res) => res,
    //     Err(err) => panic!("Quantization failed, because: {:?}", err),
    // };

    // // Enable dithering for subsequent remappings
    // res.set_dithering_level(1.0).unwrap();

    // // You can reuse the result to generate several images with the same palette
    // let (palette, pixels) = res.remapped(&mut img).unwrap();

    // alert("Done! Got palette {:?} and {} pixels with {}% quality");
    // alert("Hello, imagequant-wasm!");
}
