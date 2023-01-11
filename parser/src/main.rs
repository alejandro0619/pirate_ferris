use parser::storage::StorageHandler;
use std::fs;
fn main(){
  let input = fs::read_to_string("test.txt").unwrap();

  let (_, result) = StorageHandler::read(&input).unwrap();

  println!("{result:#?}");
}