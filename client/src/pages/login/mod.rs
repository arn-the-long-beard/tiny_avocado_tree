//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};
use shared::models::auth::AuthData;

pub type Model = Option<AuthData>;

pub enum Msg {
    Login,
    Clear,
    Logout,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login => {}
        Msg::Clear => {}
        Msg::Logout => {}
    }
}
pub fn view(auth_data: &Model) -> Node<Msg> {
    div!["login page"]
}
