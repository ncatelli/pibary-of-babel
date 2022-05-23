use pibary_of_babel::*;
use std::io::{self, Write};

fn main() {
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    print!("3.");
    for digit in Pi::new().skip(1).take(usize::MAX) {
        let _ = write!(&mut stdout_lock, "{:?}", digit);
    }
}
