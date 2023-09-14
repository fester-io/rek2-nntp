use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn authenticate(
    stream: &mut TcpStream,
    username: &str,
    password: &str,
) -> Result<(), &'static str> {
    let mut reader = BufReader::new(stream);

    let auth_command = format!("AUTHINFO USER {}\r\n", username);
    stream.write_all(auth_command.as_bytes()).unwrap();
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    // Check if server asks for password
    if !response.starts_with("381") {
        return Err("Failed to authenticate username");
    }

    let pass_command = format!("AUTHINFO PASS {}\r\n", password);
    stream.write_all(pass_command.as_bytes()).unwrap();
    response.clear();
    reader.read_line(&mut response).unwrap();

    if !response.starts_with("281") {
        return Err("Failed to authenticate password");
    }

    Ok(())
}
