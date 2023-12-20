use super::*;

pub struct ListProps {
    pub items: Vec<ListItemProps>,
}

pub fn list(props: ListProps) -> Markup {
    html! {
        @for item in props.items {
            (list_item(item))
        }
    }
}
