//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};
use shared::models::user::User;

pub type Model = Option<User>;

/// Action on register page
pub enum Msg {
    Register,
    Clear,
}
/// Update on register pages
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Register => {}
        Msg::Clear => {}
    }
}

/// view of register page
pub fn view(user_data: &Option<User>) -> Node<Msg> {
    div!["register page"]
}
