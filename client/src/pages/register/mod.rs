use seed::{prelude::*, *};
use shared::models::user::User;
pub type Model = User;

/// Action on register page
pub enum Msg {
    Register,
    PasswordChanged(String),
    UsernameChanged(String),
    EmailChanged(String),
    FirstNameChanged(String),
    LastNameChanged(String),

    Clear,
}
/// Update on register pages
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Register => {}
        Msg::Clear => {}
        Msg::PasswordChanged(text) => model.credentials.set_password(text),
        Msg::UsernameChanged(text) => model.credentials.set_username(text),
        Msg::FirstNameChanged(text) => model.first_name = text,
        Msg::LastNameChanged(text) => model.last_name = text,
        Msg::EmailChanged(text) => model.credentials.set_email(text),
    }
}

/// view of register page
pub fn view(model: &Model) -> Node<Msg> {
    div![form![
        attrs! {
        At::Action => "/api/register",
        At::Method=> "post",
        At::Target=> "_top"
        },
        label![attrs! { At::For => "username"}, "Username"],
        input![
            id!("username"),
            attrs! {
            At::Required => true,
            At::Value=> model.credentials.username(),
            At::MinLength=> "5",
            At::Name => "username",
            At::MaxLength=> "15",
            At::Type=> "text"
                    },
            input_ev(Ev::Input, Msg::UsernameChanged),
        ],
        label![attrs! { At::For => "email"}, "Email"],
        input![
            id!("email"),
            attrs! {
            At::Required => true,
            At::Value => model.credentials.email(),
            At::MinLength=> "5",
            At::MaxLength=> "15"
            At::Name => "email",
            At::Type=> "email"
               },
            input_ev(Ev::Input, Msg::EmailChanged),
        ],
        label![attrs! { At::For => "password"}, "Password"],
        input![
            id!("password"),
            attrs! {
                At::Required => true,
                At::MinLength=> "8",
                At::MaxLength=> "25"
                At::Value => model.credentials.password(),
                At::Name => "password",
                At::Type=> "password"
            },
            input_ev(Ev::Input, Msg::PasswordChanged),
        ],
        label![attrs! { At::For => "first_name"}, "First Name"],
        input![
            id!("first_name"),
            attrs! {
            At::Required => true,
            At::Name => "first_name",
            At::Type=> "text",
            At::Value=> model.first_name,
                   },
            input_ev(Ev::Input, Msg::FirstNameChanged),
        ],
        br![],
        label![attrs! { At::For => "last_name"}, "Last Name"],
        input![
            id!("last_name"),
            attrs! {
            At::Required => true,
            At::MaxLength=> "15"
            At::Name => "last_name",
            At::Type=> "text"
            At::Value=> model.last_name,
                   },
            input_ev(Ev::Input, Msg::LastNameChanged),
        ],
        br![],
        input![attrs! {
            At::MaxLength=> "15"
            At::Type=> "submit" ,
            At::Value=> "Register" ,
        }],
    ]]
}
