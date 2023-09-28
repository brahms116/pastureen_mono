use blog_site::{PostMeta, RenderedPostContentData};
use markdown::{mdast::Node, to_html_with_options, to_mdast, Constructs, Options, ParseOptions};

pub fn generate() {
    let md = include_str!("../demo.md");

    let parse_options = get_parse_options();

    let html = to_html_with_options(
        md,
        &Options {
            parse: parse_options,
            ..Default::default()
        },
    )
    .expect("Failed to parse markdown");

    let meta = extract_meta(md);
    let _post_content_data = RenderedPostContentData {
        meta,
        post_content_html: html,
    };
}

pub fn extract_meta(post: &str) -> PostMeta {
    let yaml = extract_yaml(post);
    println!("{:?}", yaml);
    if let Some(yaml) = yaml {
        let meta: PostMeta = serde_yaml::from_str(&yaml).expect("Failed to parse YAML");
        return meta;
    }
    PostMeta::default()
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

pub fn extract_yaml(post: &str) -> Option<String> {
    let ast = to_mdast(post, &get_parse_options()).unwrap();

    for node in ast.children().unwrap().into_iter() {
        match node {
            Node::Yaml(yaml) => return Some(yaml.value.clone()),
            _ => continue,
        }
    }
    None
}
