use std::fs;
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
}
