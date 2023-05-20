use readings::*;
use std::io;

fn main() -> io::Result<()> {
    let std_in = io::stdin();
    let mut input = String::new();

    println!("What do you want to do?");
    println!(       
"1. List previous entries
2. Insert new entry
3. Change location of manifest file
4. Create new manifest");
    std_in.read_line(&mut input)?;
    Ok(())
}
