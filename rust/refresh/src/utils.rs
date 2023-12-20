use super::*;

pub fn correct_alpine_directives(markup: Markup) -> Markup {
    let string = markup.into_string();
    let string = string.replace("x-on:click-", "x-on:click.");
    let string = string.replace("x-on:keydown-", "x-on:keydown.");
    let string = string.replace("ctrl-k-", "ctrl.k.");
    let string = string.replace("-window", ".window");
    PreEscaped(string)
}
