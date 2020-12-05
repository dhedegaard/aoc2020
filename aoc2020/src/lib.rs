mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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

#[wasm_bindgen]
pub fn day02_input() -> String {
    day02::get_input()
}

#[wasm_bindgen]
pub fn day02_part1(input: &str) -> String {
    day02::part1(input)
}

#[wasm_bindgen]
pub fn day02_part2(input: &str) -> String {
    day02::part2(input)
}

#[wasm_bindgen]
pub fn day03_input() -> String {
    day03::get_input()
}

#[wasm_bindgen]
pub fn day03_part1(input: &str) -> String {
    day03::part1(input)
}

#[wasm_bindgen]
pub fn day03_part2(input: &str) -> String {
    day03::part2(input)
}

#[wasm_bindgen]
pub fn day04_input() -> String {
    day04::get_input()
}

#[wasm_bindgen]
pub fn day04_part1(input: &str) -> String {
    day04::part1(input)
}

#[wasm_bindgen]
pub fn day04_part2(input: &str) -> String {
    day04::part2(input)
}

#[wasm_bindgen]
pub fn day05_input() -> String {
    day05::get_input()
}

#[wasm_bindgen]
pub fn day05_part1(input: &str) -> String {
    day05::part1(input)
}

#[wasm_bindgen]
pub fn day05_part2(input: &str) -> String {
    day05::part2(input)
}
