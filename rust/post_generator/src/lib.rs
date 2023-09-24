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

    println!("{:?}", to_mdast(md, &parse_options).unwrap())
}
