use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32{
    a + b
}

#[wasm_bindgen]
pub fn div(a: i32, b: i32) -> i32{
    a / b
}
pub fn get_number_suffix (n: i32) -> String {
    match n % 10 {
        1 => format!("{n}st"),
        2 => format!("{n}nd"),
        3 => format!("{n}rd"),
        _ => format!("{n}th")
    }
}

#[wasm_bindgen]
pub fn goat_song(count: i32) -> String {
    let mut result = String::new();
    for i in 0..count {
        let label = get_number_suffix(i);
        result = format!("{result}{label} goat's message: EXPLOSION\n");
    }
    return result.to_string();
}
