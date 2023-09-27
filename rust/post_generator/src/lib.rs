use markdown::{to_html_with_options, to_mdast, Constructs, Options, ParseOptions};

pub fn generate() {
    let md = include_str!("../demo.md");

    let parse_options = ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Default::default()
        },
        ..Default::default()
    };

    // println!("{}", to_html_with_options(md, &Options{
    //     parse: ParseOptions{
    //         constructs: Constructs{
    //             frontmatter: true,
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // }).unwrap());

    println!("{:?}", to_mdast(md, &parse_options).unwrap().to_string())
}

pub struct PostMeta {
    pub date: String,
    pub tags: Vec<String>,
}

pub fn extract_meta(post: &str) -> PostMeta {
    todo!()
}

fn get_parse_options() -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn extract_yaml(post: &str) -> String {
    let ast = to_mdast(post, &get_parse_options()).unwrap();
    todo!()
}
