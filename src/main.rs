use pibary_of_babel::*;
use sliding_windows::{IterExt, Storage};

fn main() {
    let input = "pi";
    let data = input.as_bytes();
    let mut storage: Storage<u8> = Storage::new(data.len());

    let found = ByteGenerator::new()
        .sliding_windows(&mut storage)
        .position(|window| window == data);

    match found {
        Some(pos_of_data) => println!("{}\n{}..{}", input, pos_of_data, pos_of_data + data.len()),
        _ => println!("not found: {:?}", &data),
    };
}
