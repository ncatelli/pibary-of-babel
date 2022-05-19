use pibary_of_babel::*;
use sliding_windows::{IterExt, Storage};

fn main() {
    let input = "ab";
    let data = input.as_bytes();
    let mut storage: Storage<u8> = Storage::new(data.len());

    let found = ByteGenerator::new()
        .take(usize::MAX / 1000000000000000)
        .sliding_windows(&mut storage)
        .position(|window| window == data);

    match found {
        Some(pos_of_data) => println!("{}\n{}..{}", input, pos_of_data, pos_of_data + data.len()),
        _ => println!("not found: {:?}", &data),
    };
}
