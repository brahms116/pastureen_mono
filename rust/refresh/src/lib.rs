use maud::{html, Markup, PreEscaped};

mod actionable;
mod close_svg;
mod deps;
mod htmx_types;
mod layout;
mod list;
mod list_item;
mod loader;
mod menu;
mod menu_item;
mod search_svg;
mod utils;
mod global_search;

use deps::*;

pub use actionable::*;
pub use close_svg::*;
pub use htmx_types::*;
pub use layout::*;
pub use list::*;
pub use list_item::*;
pub use loader::*;
pub use menu::*;
pub use menu_item::*;
pub use search_svg::*;
pub use utils::*;
pub use global_search::*;

