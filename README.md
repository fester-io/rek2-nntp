# ReK2 NNTP RFC3977/RFC4643 Rust Library
- By Chris F.N. aka ReK2

`rek2_nntp` is a Rust asynchronous NNTP client library compliant with [RFC 3977](https://datatracker.ietf.org/doc/html/rfc3977) and [RFC 4643](https://datatracker.ietf.org/doc/html/rfc4643). It allows you to connect to NNTP servers to read, list, and post articles securely via TLS/SSL.

## Features

- ✅ Authentication (with TLS/SSL)
- ✅ Listing available newsgroups (`LIST`)
- ✅ Reading articles (`ARTICLE`)
- ✅ Posting articles (`POST`)
- ✅ Selecting newsgroups (`GROUP`)
- ✅ Retrieving new newsgroups (`NEWGROUPS`)
- ✅ Retrieving article headers (`HEAD`)
- ✅ Retrieving article bodies (`BODY`)
- ✅ Retrieving article statistics (`STAT`)
- ✅ Graceful session termination (`QUIT`)
- ⚠️ Retrieving new articles (`NEWNEWS`) *(often disabled on modern servers, use with caution)*

## Installation

Add the following to your project's `Cargo.toml`:

```toml
[dependencies]
rek2_nntp = "1.0.0"  # Replace with the latest version
```

Then, run:

```sh
cargo build
```

## Usage

### Authentication

```rust
use rek2_nntp::authenticate;

#[tokio::main]
async fn main() {
    let result = authenticate("news.example.com", "username", "password").await;
    match result {
        Ok(mut connection) => {
            println!("Successfully authenticated!");
            // You can now use `connection` with other commands
        }
        Err(err) => {
            eprintln!("Failed to authenticate: {}", err);
        }
    }
}
```

### Listing Newsgroups

```rust
use rek2_nntp::{authenticate, list_newsgroups};

let mut connection = authenticate("news.example.com", "username", "password").await?;

let newsgroups = list_newsgroups(&mut connection).await?;
for group in newsgroups.iter().take(10) {
    println!("{} ({}-{}) Status: {}", group.name, group.low, group.high, group.status);
}
```

### Reading Articles from a Newsgroup

```rust
use rek2_nntp::{authenticate, read_from_group};

let mut connection = authenticate("news.example.com", "username", "password").await?;

// Read first 5 articles from "comp.lang.rust"
let articles = read_from_group(&mut connection, "comp.lang.rust", Some((1, 5))).await?;
for article in articles {
    println!("Header: {}\nBody: {}", article.header, article.body);
}
```

### Posting an Article

```rust
use rek2_nntp::{authenticate, post_to_group, PostArticle};

let mut connection = authenticate("news.example.com", "username", "password").await?;

let article = PostArticle {
    from: "user@example.com".to_string(),
    newsgroups: "comp.lang.rust".to_string(),
    subject: "Hello Rust!".to_string(),
    body: "This is a test article posted using rek2_nntp".to_string(),
};

post_to_group(&mut connection, &article, &"comp.lang.rust".to_string()).await?;
println!("Article posted successfully!");
```

### Selecting a Newsgroup (`GROUP`)

```rust
use rek2_nntp::{authenticate, group};

let mut connection = authenticate("news.example.com", "username", "password").await?;

let response = group(&mut connection, "comp.lang.c").await?;
println!("Server Response: {}", response);
```

### Retrieving New Newsgroups (`NEWGROUPS`)

```rust
use rek2_nntp::{authenticate, newgroups};

let mut connection = authenticate("news.example.com", "username", "password").await?;

let groups = newgroups(&mut connection, "20250101", "000000", None).await?;
for group in groups.iter().take(5) {
    println!("Newsgroup: {}", group.name);
}
```

### Retrieving Article Headers (`HEAD`)

```rust
use rek2_nntp::{authenticate, head, group};

let mut connection = authenticate("news.example.com", "username", "password").await?;
let _ = group(&mut connection, "comp.lang.c").await?;

let header = head(&mut connection, "1").await?;
println!("Header: {}", header);
```

### Retrieving Article Bodies (`BODY`)

```rust
use rek2_nntp::{authenticate, body, group};

let mut connection = authenticate("news.example.com", "username", "password").await?;
let _ = group(&mut connection, "comp.lang.c").await?;

let article_body = body(&mut connection, "1").await?;
println!("Body: {}", article_body);
```

### Retrieving Article Statistics (`STAT`)

```rust
use rek2_nntp::{authenticate, stat, group};

let mut connection = authenticate("news.example.com", "username", "password").await?;
let _ = group(&mut connection, "comp.lang.c").await?;

let message_id = stat(&mut connection, "1").await?;
println!("Message ID: {}", message_id);
```

### Gracefully Ending the Session (`QUIT`)

```rust
use rek2_nntp::{authenticate, quit};

let mut connection = authenticate("news.example.com", "username", "password").await?;
quit(&mut connection).await?;
println!("Session closed.");
```

## Running Integration Tests

Set environment variables before running tests:

```sh
export NNTP_HOST=news.example.com
export NNTP_USERNAME=username
export NNTP_PASSWORD=password
```

Then run:

```sh
cargo test -- --nocapture
```

## Contributing

Contributions are welcome!

- Issues and pull requests: [~rek2/rek2_nntp](https://git.sr.ht/~rek2/rek2_nntp)
- Mailing list for patches: [~rek2/rek2_nntp@lists.sr.ht](https://lists.sr.ht/~rek2/rek2_nntp)

## License

This library is licensed under the [GNU GPL v3](LICENSE).

