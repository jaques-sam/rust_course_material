use std::fs::File;
use std::io::{self, Read};

/// The long version
fn read_line_from_file_long() -> Result<String, io::Error> {
    let f = File::open("examples/username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/// The short version
fn read_line_from_file() -> Result<String, io::Error> {
    let mut file = File::open("examples/username.txt")?;  //--> propagate error
    let mut line = String::new();
    file.read_to_string(&mut line)?;
    Ok(line)
}

fn main() -> Result<(), io::Error> {
    match read_line_from_file_long() {
        Ok(s) => println!("Username is {}", s),
        _ => println!("Failed to read username from file!"),
    };

    let username = read_line_from_file()?;
    println!("Username is {}", username);


    let _ = std::fs::read_to_string("file_not_found.txt")?;

    Ok(())
}
