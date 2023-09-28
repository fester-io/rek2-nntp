use std::error::Error;
use std::io::Read;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::str;

pub struct Article {
    pub from: String,
    pub subject: String,
    pub body: String,
}

pub fn post_to_group(
    stream: &mut TcpStream,
    article: &Article,
    newsgroup: &String,
) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream.try_clone()?);

    let group_command = format!("GROUP {}\r\n", newsgroup);
    println!("Group Command: {}", group_command);
    stream.write_all(group_command.as_bytes())?;
    stream.flush()?;

    let mut response = String::new();
    let mut buffer = [0; 1024]; // 1 KB buffer
    let bytes_read = stream.read(&mut buffer)?;

    // Attempt to convert to UTF-8
    match str::from_utf8(&buffer[0..bytes_read]) {
        Ok(valid_str) => {
            response.push_str(valid_str);
            println!("GROUP response: {}", response);
        }
        Err(e) => {
            println!("Failed to convert to UTF-8: {}", e);
            // Handle the error as you see fit
            // Hex Dump for Debugging
            println!("Hex Dump: {:?}", &buffer[0..bytes_read]);

            // Ignore Invalid UTF-8 Sequences
            let lossy_str = String::from_utf8_lossy(&buffer[0..bytes_read]);
            println!("Lossy Conversion: {}", lossy_str);
        }
    }
    response.push_str(std::str::from_utf8(&buffer[0..bytes_read])?);
    println!("GROUP response: {}", response);

    // Check for the 211 response code
    if !response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to set the newsgroup",
        )));
    }

    // POST command
    let post_command = "POST\r\n";
    writer.write_all(post_command.as_bytes())?;
    writer.flush()?;

    response.clear();
    reader.read_line(&mut response)?;
    println!("POST response: {}", response);

    // Article data
    let article_data = format!(
        "From: {}\r\nSubject: {}\r\n\r\n{}\r\n.\r\n",
        article.from, article.subject, article.body
    );
    writer.write_all(article_data.as_bytes())?;
    writer.flush()?;

    response.clear();
    reader.read_line(&mut response)?;
    println!("Article data response: {}", response);

    Ok(())
}
