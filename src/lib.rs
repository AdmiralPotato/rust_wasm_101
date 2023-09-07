use serde::Serialize;
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

#[derive(Clone, Debug, Serialize)]
#[wasm_bindgen]
pub struct Goat {
    name: String,
    pub power_level: i32,
    pub is_grumpy: bool
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct GoatIterator {
    goats: Vec<Goat>,
    index: usize,
}

#[wasm_bindgen]
impl GoatIterator {
    pub fn next_goat(&mut self) -> Option<Goat> {
        if self.index >= self.goats.len() { None }
        else {
            let ret = self.goats[self.index].clone();
            self.index += 1;
            Some(ret)
        }
    }
    pub fn has_another_goat(&self) -> bool {
        self.index < self.goats.len()
    }
}

#[wasm_bindgen]
impl Goat {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    fn parse_log_internal(goat_log: String) -> Vec<Goat> {
        console_error_panic_hook::set_once();
        let mut result = vec!();
        let lines = goat_log.trim().split("\n");
        let mut name_opt: Option<String> = None;
        let mut power_level_opt: Option<i32> = None;
        let mut is_grumpy_opt: Option<bool> = None;
        for line in lines {
            let mut line = line.trim().split(':');
            let key = line.next();
            let value = line.next();
            // TODO: Implement log output on the JS side?
            println!("key: '{key:?}', value: '{value:?}'");
            match (key, value) {
                (Some(key), Some(value)) => {
                    let value = value.trim();
                    match key.trim() {
                        "name" => name_opt = Some(value.to_string()),
                        "power_level" => power_level_opt = Some(value.parse::<i32>().expect("bad number")),
                        "is_grumpy" => is_grumpy_opt = match value {
                            "true" => Some(true),
                            "false" => Some(false),
                            _ => None
                        },
                        _ => println!("Invalid key {key:?}")
                    }
                    match (&name_opt, &power_level_opt, &is_grumpy_opt) {
                        (Some(name), Some(power_level), Some(is_grumpy)) => {
                            println!("Goat is ready!");
                            result.push(Goat{
                                name: name.clone(),
                                power_level: *power_level,
                                is_grumpy: *is_grumpy
                            });
                            name_opt = None;
                            power_level_opt = None;
                            is_grumpy_opt = None;
                        },
                        _ => println!("Goat not ready yet")
                    }
                },
                _ => println!("Missing key or value")
            }
        }
        result
    }
    pub fn parse_log_as_iter(goat_log: String) -> GoatIterator {
        let result = Self::parse_log_internal(goat_log);
        GoatIterator {
            goats: result,
            index: 0,
        }
    }
    pub fn parse_log_as_json(goat_log: String) -> String {
        let result = Self::parse_log_internal(goat_log);
        serde_json::to_string(&result).unwrap()
    }
}
