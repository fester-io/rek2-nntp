use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub struct Article {
    pub subject: String,
    pub body: String,
}

pub fn post_to_group(
    stream: &mut TcpStream,
    group: &str,
    article: &Article,
) -> Result<(), &'static str> {
    let mut reader = BufReader::new(stream);

    let group_command = format!("GROUP {}\r\n", group);
    stream.write_all(group_command.as_bytes()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("211") {
        return Err("Failed to select group");
    }

    let post_command = "POST\r\n";
    stream.write_all(post_command.as_bytes()).unwrap();
    response.clear();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("340") {
        return Err("Server not ready to accept article");
    }

    let article_data = format!(
        "Subject: {}\r\n\r\n{}\r\n.\r\n",
        article.subject, article.body
    );
    stream.write_all(article_data.as_bytes()).unwrap();
    response.clear();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("240") {
        return Err("Failed to post article");
    }

    Ok(())
}
