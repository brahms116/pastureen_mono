use super::*;

pub enum Actionable {
    Link(String),
    Htmx(HtmxOptions),
    Alpine(String),
}
