use super::*;


pub struct MenuProps {
    pub sections: Vec<MenuSectionProps>,
}

pub struct MenuSectionProps {
    pub label: String,
    pub items: Vec<MenuItemProps>,
}

pub fn menu(props: MenuProps) -> Markup {
    html! {
        .menu {
            .menu__secitons {
                @for section in props.sections {
                    (menu_section(section))
                }
            }
        }
    }
}

pub fn menu_section(props: MenuSectionProps) -> Markup {
    html! {
        .menu-section {
            .menu-section__label {
                (props.label)
            }
            .menu-section__items {
                @for item in props.items {
                    (menu_item(item))
                }
            }
        }
    }
}
