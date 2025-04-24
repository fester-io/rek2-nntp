//! Authentication module for NNTP servers.
//!
//! Types:
//! - `AuthType`: Plain or SSL authentication modes.
//!
//! Structs:
//! - `AuthenticatedConnection`: wraps a TLS stream after successful login.
//!
//! Functions:
//! - `authenticate()`: connects to an NNTP server and performs AUTHINFO USER/PASS.
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpStream,
};
use tokio_native_tls::TlsConnector;

pub enum AuthType {
    Plain,
    SSL,
}

pub struct AuthenticatedConnection {
    pub tls_stream: tokio_native_tls::TlsStream<TcpStream>,
}

pub async fn authenticate(
    host: &str,
    username: &str,
    password: &str,
) -> Result<AuthenticatedConnection, Box<dyn std::error::Error>> {
    let connector = TlsConnector::from(native_tls::TlsConnector::new()?);
    let address: String = format!("{}:563", host);
    let stream = TcpStream::connect(address).await?;
    let tls_stream = connector.connect(host, stream).await?;

    let mut reader = BufReader::new(tls_stream);

    let user_command = format!("AUTHINFO USER {}\r\n", username);
    reader.get_mut().write_all(user_command.as_bytes()).await?;
    reader.get_mut().flush().await?;

    let response = crate::utils::wait_for_response(&mut reader, &["381"], 5, 3).await?;

    if response.starts_with("381") {
        let pass_command = format!("AUTHINFO PASS {}\r\n", password);
        reader.get_mut().write_all(pass_command.as_bytes()).await?;
        reader.get_mut().flush().await?;

        let response = crate::utils::wait_for_response(&mut reader, &["281"], 5, 3).await?;
        println!("Response after AUTHINFO PASS: {}", response);

        if response.starts_with("281") {
            Ok(AuthenticatedConnection {
                tls_stream: reader.into_inner(),
            })
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Authentication failed",
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Unexpected response to AUTHINFO USER",
        )))
    }
}
