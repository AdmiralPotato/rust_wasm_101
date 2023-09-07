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

#[derive(Debug)]
pub struct Goat {
    pub name: String,
    pub power_level: i32,
    pub is_grumpy: bool
}
impl Goat {
    pub fn parse_log(goat_log: String) -> Vec<Goat> {
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
                        _ => println!("Invalid key")
                    }
                    match(&name_opt, &power_level_opt, &is_grumpy_opt) {
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
}

#[wasm_bindgen]
pub fn parse_goat_log(goat_log: String) -> String {
    // TODO: Return Vec<Goat>
    let result = Goat::parse_log(goat_log);
    format!("Debug! {result:?}")
}
