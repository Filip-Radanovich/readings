use readings::*;
use std::io;



fn main() -> io::Result<()> {
    let std_in = io::stdin();
    let mut input = String::new();
    application(&std_in, &mut input);
    Ok(())
}
