use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn list_subscribed_groups(stream: &mut TcpStream) -> Result<Vec<String>, &'static str> {
    let mut reader = BufReader::new(stream);
    let mut groups = Vec::new();

    let list_command = "LIST\r\n";
    stream.write_all(list_command.as_bytes()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("215") {
        return Err("Failed to list groups");
    }

    loop {
        response.clear();
        reader.read_line(&mut response).unwrap();
        if response == ".\r\n" {
            break;
        }
        let group_name = response.split_whitespace().next().unwrap().to_string();
        groups.push(group_name);
    }

    Ok(groups)
}

pub fn subscribe_to_group(stream: &mut TcpStream, group: &str) -> Result<(), &'static str> {
    // In NNTP, subscribing is essentially just selecting a group.
    // So we can reuse the logic from read.rs to select a group.
    let mut reader = BufReader::new(stream);

    let group_command = format!("GROUP {}\r\n", group);
    stream.write_all(group_command.as_bytes()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("211") {
        return Err("Failed to subscribe to group");
    }

    Ok(())
}

pub fn unsubscribe_from_group() -> Result<(), &'static str> {
    // NNTP doesn't have a specific command for unsubscribing from a group.
    // The client simply stops issuing commands for that group.
    Ok(())
}
