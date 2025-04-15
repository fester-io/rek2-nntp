pub mod auth;
pub mod list;
pub mod post;
pub mod read;
pub mod utils;

pub use auth::{authenticate, AuthType, AuthenticatedConnection};
pub use list::list_newsgroups;
pub use post::{post_to_group, Article as PostArticle};
pub use read::{read_from_group, Article as ReadArticle};
