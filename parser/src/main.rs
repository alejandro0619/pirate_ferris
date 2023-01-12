use parser::storage::StorageHandler;
use std::fs;
fn main() {
    //let input = fs::read_to_string("test.txt").unwrap();

    let (_, result) =
        StorageHandler::parse_datetime("timestamp = 2023-01-12T02:53:00.999338645+00:00\n")
            .unwrap();

    println!("{result:#?}");
}

