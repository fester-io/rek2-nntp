use bufstream::BufStream;
use std::error::Error;
use std::io::{BufRead, Write};
use std::net::TcpStream;

pub struct Article {
    pub subject: String,
    pub body: String,
}

pub fn post_to_group(stream: &mut TcpStream, article: &Article) -> Result<(), Box<dyn Error>> {
    let mut buf_stream = BufStream::new(stream);
    let post_command = "POST\r\n";
    buf_stream.write_all(post_command.as_bytes())?;
    buf_stream.flush()?;

    let mut response = String::new();
    buf_stream.read_line(&mut response)?;

    if !response.starts_with("340") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "POST command failed",
        )));
    }

    let article_data = format!("Subject: {}\r\n\r\n{}", article.subject, article.body);
    buf_stream.write_all(article_data.as_bytes())?;
    buf_stream.flush()?;

    response.clear();
    buf_stream.read_line(&mut response)?;

    if !response.starts_with("240") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Article post failed",
        )));
    }

    Ok(())
}
