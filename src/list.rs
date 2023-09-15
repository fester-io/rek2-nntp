use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Newsgroup {
    pub name: String,
    pub high: u32,
    pub low: u32,
    pub status: String,
}

pub fn list_newsgroups(stream: &mut TcpStream) -> Result<Vec<Newsgroup>, Box<dyn Error>> {
    let list_command = "LIST\r\n";
    stream.write_all(list_command.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;

    if !response.starts_with("215") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to execute LIST command",
        )));
    }

    let mut newsgroups = Vec::new();

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        if line == ".\r\n" {
            break;
        }

        let parts: Vec<&str> = line.trim().split(" ").collect();
        if parts.len() >= 4 {
            let newsgroup = Newsgroup {
                name: parts[0].to_string(),
                high: parts[1].parse()?,
                low: parts[2].parse()?,
                status: parts[3].to_string(),
            };
            newsgroups.push(newsgroup);
        }
    }

    Ok(newsgroups)
}
