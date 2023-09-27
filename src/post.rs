use bufstream::BufStream;
use std::error::Error;
use std::io::{BufRead, Write};
use std::net::TcpStream;

pub struct Article {
    pub from: String,
    // pub newsgroup: String,
    pub subject: String,
    //  pub subject: Vec<u8>,
    pub body: String,
    //pub body: Vec<u8>,
}

pub fn post_to_group(
    stream: &mut TcpStream,
    article: &Article,
    newsgroup: &String,
) -> Result<(), Box<dyn Error>> {
    let newsgroup_header = format!("Newsgroups: {}\r\n", newsgroup);
    let group_command = format!("GROUP {}\r\n", newsgroup);
    let mut buf_stream = BufStream::new(stream);
    // let group_command: Vec<u8> = format!("GROUP {}\r\n", newsgroup).into_bytes();
    let post_command = "POST\r\n";

    // Debug
    println!("[Debug]: The newsgroup_header here: {}", newsgroup_header);
    println!("[Debug]: The POST command here: {}", post_command);
    println!("[Debug]: The GROUP command here: {:?}", group_command);
    println!("[Debug]: The BUFF stream: {:?}", buf_stream);

    // here lets tell the server what group we going to post to
    //buf_stream.write_all(group_command.as_bytes())?;
    buf_stream.write_all(group_command.as_bytes())?;
    buf_stream.flush()?;

    let mut response = String::new();
    buf_stream.read_line(&mut response)?;

    println!("[Debug]: response from GROUP command: {}", response);
    // Check server response after GROUP command
    if !response.starts_with("211") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "GROUP command failed",
        )));
    }

    buf_stream.write_all(post_command.as_bytes())?;
    buf_stream.flush()?;

    let mut response = String::new();
    buf_stream.read_line(&mut response)?;

    println!("[Debug]: Server response after POST command: {}", response);

    if !response.starts_with("340") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "POST command failed",
        )));
    }

    // Debug
    println!(
        "[Debug]: Article Subject (before UTF-8 conversion): {:?}",
        article.subject
    );
    println!(
        "[Debug]: Article Body (before UTF-8 conversion): {:?}",
        article.body
    );
    // Convert the subject and body to UTF-8 if not already
    let _from_utf8 = String::from_utf8_lossy(article.from.as_bytes());

    let subject_utf8 = String::from_utf8_lossy(article.subject.as_bytes());
    let body_utf8 = String::from_utf8_lossy(article.body.as_bytes());

    println!(
        "[Debug]: Article Subject (after UTF-8 conversion): {:?}",
        subject_utf8
    );
    println!(
        "[Debug]: Article Body (after UTF-8 conversion): {:?}",
        body_utf8
    );

    let article_data = format!(
        "Subject: {}\r\n{}\r\n\r\n{}\r\n.\r\n",
        subject_utf8, newsgroup_header, body_utf8
    );

    println!("[Debug]: Article data bytes: {:?}", article_data.as_bytes());

    println!("[Debug]: Sending article data:\n{}", article_data);

    buf_stream.write_all(article_data.as_bytes())?;
    buf_stream.flush()?;

    response.clear();
    buf_stream.read_line(&mut response)?;
    println!(
        "[Debug]: Server response after sending article: {}",
        response
    );

    if !response.starts_with("240") {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Article post failed",
        )));
    }

    Ok(())
}
