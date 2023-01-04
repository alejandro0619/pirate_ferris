use parser::parse;
use std::fs;
fn main() {
  let input = fs::read_to_string("example.txt").expect("didn't find the file");
    parse(&input);
}
