use super::contracts;

mod svgs;
mod utils;

mod index_page;
mod post_page;
mod posts_page;


pub use index_page::render_index_page;
pub use post_page::{render_post_page, render_post};
pub use posts_page::render_posts_page;
