use pibary_of_babel::*;
use sliding_windows::{IterExt, Storage};

const USAGE: &str = "pibary-of-babel <input>";

fn main() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let arg_len = args.len();

    let input = match arg_len {
        1 => args.get(0).ok_or_else(|| USAGE.to_string()),
        _ => Err(USAGE.to_string()),
    }?;

    let data = input.as_bytes();
    let mut storage: Storage<u8> = Storage::new(data.len());

    let found = ByteGenerator::new()
        .take(usize::MAX)
        .sliding_windows(&mut storage)
        .position(|window| window == data);

    match found {
        Some(pos_of_data) => {
            println!("{}..{}", pos_of_data, pos_of_data + data.len());
            Ok(())
        }
        _ => Err(format!("not found: {:?}", &data)),
    }
}
