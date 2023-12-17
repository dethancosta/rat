use std::io::{self, BufReader, BufRead, Read};

pub fn get_contents<T: Read>(source: T) -> Result<(), io::Error> {
    let mut reader = BufReader::new(source);
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
