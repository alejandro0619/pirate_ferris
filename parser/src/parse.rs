use parser::storage::{Storage, StorageHandler};
use std::fs;
fn main() {

      println!("Before appending...\n");
      let input = fs::read_to_string("example.txt").expect("cannot find the file");

      StorageHandler::read(&input);

      println!("Appending...");
      StorageHandler::write(&Storage::new(48, "pastae", 1));

      println!("After appending...\n");
      let input2 = fs::read_to_string("example.txt").expect("cannot find the file");
      StorageHandler::read(&input2);

    
    
}
