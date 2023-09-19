use bufstream::BufStream;
use native_tls::TlsConnector;
use std::error::Error;
use std::io::{BufRead, Write};
use std::net::TcpStream;

pub enum AuthType {
    Plain,
    SSL,
}

pub trait ReadWrite: BufRead + Write {}
impl<T: BufRead + Write> ReadWrite for T {}

pub fn authenticate(
    stream: &mut TcpStream,
    host: &str,
    username: &str,
    password: &str,
    auth_type: AuthType,
) -> Result<(), Box<dyn Error>> {
    let mut stream: Box<dyn ReadWrite> = match auth_type {
        AuthType::Plain => Box::new(BufStream::new(stream.try_clone()?)),
        AuthType::SSL => {
            let connector = TlsConnector::new().map_err(Box::new)?;
            let tls_stream = connector.connect(host, stream.try_clone()?).map_err(|e| {
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    e.to_string(),
                ))
            })?;
            Box::new(BufStream::new(tls_stream))
        }
    };

    let mut buffer = Vec::new();
    let user_command = format!("AUTHINFO USER {}\r\n", username);
    stream.write_all(user_command.as_bytes())?;
    stream.flush()?;
    stream.read_until(b'\n', &mut buffer)?;

    buffer.clear();
    let pass_command = format!("AUTHINFO PASS {}\r\n", password);
    stream.write_all(pass_command.as_bytes())?;
    stream.flush()?;
    stream.read_until(b'\n', &mut buffer)?;

    Ok(())
}
