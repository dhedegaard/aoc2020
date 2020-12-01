mod utils;

mod day01;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn day01_input() -> String {
    day01::get_input()
}

#[wasm_bindgen]
pub fn day01_part1(input: &str) -> String {
    day01::part1(input)
}

#[wasm_bindgen]
pub fn day01_part2(input: &str) -> String {
    day01::part2(input)
}
