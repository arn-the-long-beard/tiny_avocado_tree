use seed::{prelude::*, *};
use shared::models::user::User;
pub type Model = User;

struct InputModel {
    field: String,
    value: String,
}

/// Action on register page
pub enum Msg {
    Register,
    Verify(String),
    Clear,
}
/// Update on register pages
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Register => {}
        Msg::Clear => {}
        Msg::Verify(text) => {
            log!(model.email);
            log!("--------------");
            log!(text);

            model.email = text;
            log!(model.email);
            log!("--------------");
        }
    }
}

/// view of register page
pub fn view(model: &Model) -> Node<Msg> {
    div![form![
        label![attrs! { At::For => "username"}, "Username"],
        input![
            id!("username"),
            attrs! {

                At::Name => "username",
                At::Type=> "text"
            }
        ],
        label![attrs! { At::For => "email"}, "Email"],
        input![
            id!("email"),
            attrs! {
                At::Value => model.email,
                At::Name => "email",
                At::Type=> "email"
            },
            input_ev(Ev::Input, Msg::Verify),
        ],
        label![attrs! { At::For => "first_name"}, "First Name"],
        input![
            id!("first_name"),
            attrs! {
                At::Name => "first_name",
                At::Type=> "text"
            }
        ],
        br![],
        label![attrs! { At::For => "last_name"}, "Last Name"],
        input![
            id!("last_name"),
            attrs! {
                At::Name => "last_name",
                At::Type=> "text"
            }
        ],
        br![],
    ]]
}
