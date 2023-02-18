use std::fs;

pub fn read_input(day: i32) -> String {
    let path = [".\\src\\inputs\\day", &day.to_string(), ".txt"].join("");
    return fs::read_to_string(path).expect("Error").trim().to_string();
}
