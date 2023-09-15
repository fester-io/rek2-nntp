use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Article {
    pub subject: String,
    pub body: String,
}

pub fn post_to_group(
    mut stream: TcpStream,
    group: &str,
    article: &Article,
) -> Result<(), Box<dyn Error>> {
    let group_command = format!("GROUP {}\r\n", group);
    stream.write_all(group_command.as_bytes())?;
    stream.flush()?;

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;

    if !response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to select group",
        )));
    }

    let post_command = "POST\r\n";
    stream.write_all(post_command.as_bytes())?;
    response.clear();
    reader.read_line(&mut response)?;

    if !response.starts_with("340") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Server not ready to accept article",
        )));
    }

    let article_data = format!(
        "Subject: {}\r\n\r\n{}\r\n.\r\n",
        article.subject, article.body
    );
    stream.write_all(article_data.as_bytes())?;
    response.clear();
    reader.read_line(&mut response)?;

    if !response.starts_with("240") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to post article",
        )));
    }

    Ok(())
}
