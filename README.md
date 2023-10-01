# ReK2 NNTP RFC4643 RFC3977 Library

This is a Rust library that provides a way to interact with NNTP servers, compliant with [RFC 3977](https://datatracker.ietf.org/doc/html/rfc3977) and [RFC 4643](https://datatracker.ietf.org/doc/html/rfc4643).

## Features

- Article Posting
- Newsgroup Listing
- Article Reading from a Group
- Authentication (Plain and SSL)

## Installation

Add the following to your `Cargo.toml`:

\```toml
[dependencies]
rek2_nntp = "0.1.1"  # Replace with the actual version
\```

Run `cargo build` to build the dependencies.

## Usage

### Importing the Library

First, add the following import to your code:

\```rust
extern crate rek2_nntp;
\```

### Authentication

To authenticate, use the `authenticate` function:

\```rust
use rek2_nntp::authenticate;
use rek2_nntp::AuthType;

// Example of how to authenticate using the library
let result = authenticate("host.com", "username", "password").await;
match result {
    Ok(connection) => {
        // Do something with the authenticated connection
    }
    Err(err) => {
        println!("Failed to authenticate: {}", err);
    }
}
\```

### Listing Newsgroups

To list newsgroups, use the `list_newsgroups` function:

\```rust
use rek2_nntp::list_newsgroups;

// ... (authenticate)

let newsgroups = list_newsgroups(&mut stream).unwrap();
\```

### Reading from a Group

To read articles from a newsgroup:

\```rust
use rek2_nntp::read_from_group;

// ... (authenticate)

let articles = read_from_group(&mut stream, "group.name", None).unwrap();
\```

### Posting to a Group

To post an article to a newsgroup:

\```rust
use rek2_nntp::post_to_group;
use rek2_nntp::Article;

// ... (authenticate)

let article = Article {
    from: "from@example.com".to_string(),
    subject: "Hello".to_string(),
    body: "World".to_string(),
};

let result = post_to_group(&mut authenticated_connection, &article, &"newsgroup.name".to_string()).await;
match result {
    Ok(_) => println!("Posted successfully"),
    Err(err) => println!("Failed to post: {}", err),
}
\```

## Contributing

Feel free to contribute to this project by creating issues, pull requests or improving the documentation.

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

