# ReK2 NNTP RFC4643 RFC3977 Library

This is a Rust library that provides a way to interact with NNTP servers, compliant with [RFC 3977](https://datatracker.ietf.org/doc/html/rfc3977) and [RFC 4643](https://datatracker.ietf.org/doc/html/rfc4643).

## Features

- Article Posting
- Newsgroup Listing
- Article Reading from a Group
- Authentication (Plain and SSL)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rek2_nntp = "0.1.0"  # Replace with the actual version
```

Run `cargo build` to build the dependencies.

## Usage

### Importing the Library

First, add the following import to your code:

```rust
extern crate rek2_nntp;
```

### Authentication

To authenticate, use the `authenticate` function:

```rust
use rek2_nntp::authenticate;
use rek2_nntp::AuthType;

// ... (connect to server and get a TcpStream)

authenticate(&mut stream, "host.com", "username", "password", AuthType::Plain).unwrap();
```

### Listing Newsgroups

To list newsgroups, use the `list_newsgroups` function:

```rust
use rek2_nntp::list_newsgroups;

// ... (authenticate)

let newsgroups = list_newsgroups(&mut stream).unwrap();
```

### Reading from a Group

To read articles from a newsgroup:

```rust
use rek2_nntp::read_from_group;

// ... (authenticate)

let articles = read_from_group(&mut stream, "group.name", None).unwrap();
```

### Posting to a Group

To post an article to a newsgroup:

```rust
use rek2_nntp::post_to_group;

// ... (authenticate)

let article_data = "Your article data here";
post_to_group(&mut stream, article_data).unwrap();
```

## Contributing

Feel free to contribute to this project by creating issues, pull requests or improving the documentation.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).


