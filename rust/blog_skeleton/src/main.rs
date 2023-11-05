use refresh_blog::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

fn get_file_descriptor(path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect(&format!("Could not open file {}", path))
}

fn build() {
    let blog_config = BlogConfig::from_env().unwrap_or_else(|e| {
        println!("Could not load config from env : {:?}", e);
        std::process::exit(1);
    });

    let mut index_file = get_file_descriptor("./build/index.html");

    index_file
        .write_all(render_index_page(blog_config.clone()).as_bytes())
        .expect("Could not write to index file");
}

fn main() {
    build()
}
