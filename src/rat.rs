use std::{io::{self, BufReader, BufRead}, fs::File};

pub fn get_contents(filename: &String) -> Result<(), io::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    loop {
        let result = reader.read_line(&mut buf)?;
        if result == 0 {
            break;
        }
        print!("{}", buf);
    }
    
    Ok(())
}
