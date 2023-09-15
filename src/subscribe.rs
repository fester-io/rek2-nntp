use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn list_subscribed_groups(mut stream: TcpStream) {
    let command = "LIST\r\n";
    stream.write_all(command.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    println!("Server Response: {}", response);
}

pub fn subscribe_to_group(group_name: &str, mut stream: TcpStream) {
    let command = format!("GROUP {}\r\n", group_name);
    stream.write_all(command.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();
    println!("Server Response: {}", response);
}

pub fn unsubscribe_from_group(group_name: &str, mut stream: TcpStream) {
    // Unsubscribing from a group in NNTP usually means just leaving it,
    // so you might not need to send any specific command to the server.
    // For this example, we'll just print a message.
    println!("Unsubscribed from group: {}", group_name);
}
