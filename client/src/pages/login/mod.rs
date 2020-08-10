use crate::request::RequestState;
use crate::Msg as RootMsg;
use seed::{prelude::*, *};
use shared::models::auth::LoginCredentials;
use shared::models::user::LoggedUser;

#[derive(Default)]
pub struct Model {
    credentials: LoginCredentials,
    request_state: RequestState<LoggedUser>,
}

pub enum Msg {
    Login,
    LoginSucceed(LoggedUser),
    LoginFailed { message: String, code: String },
    PasswordChanged(String),
    TargetChanged(String),
    Clear,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Login => {
            model.request_state = RequestState::IsPending(true);
            let request = Request::new("/api/auth")
                .method(Method::Post)
                .json(&model.credentials)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    Msg::LoginSucceed(response.json().await.unwrap())
                } else {
                    Msg::LoginFailed {
                        message: response.text().await.unwrap(),
                        code: response.status().code.to_string(),
                    }
                }
            });
        }
        Msg::Clear => {}
        Msg::LoginSucceed(logged_user) => {
            model.request_state = RequestState::Success(logged_user.clone());
            orders.notify(logged_user.clone());
            log!("notify {:?}", logged_user.clone());
        }
        Msg::LoginFailed { message, code } => {
            model.request_state = RequestState::Failed { message, code }
        }
        Msg::PasswordChanged(pwd) => {
            model.credentials.set_password(pwd);
        }
        Msg::TargetChanged(target) => model.credentials.set_target(target),
    }
}
pub fn view(model: &Model) -> Node<Msg> {
    match &model.request_state {
        RequestState::Success(user) => div![p![
            C!["centred"],
            "Welcome ",
            style! {St::Color => "darkblue"},
            user.username(),
            ". :)"
        ]],
        RequestState::IsPending(status) => form(model, status),
        RequestState::Failed { message, code } => p![
            C!["centred"],
            format!("An error happened {} with the code {}", message, code)
        ],
    }
}

fn form(model: &Model, status: &bool) -> Node<Msg> {
    form![
        ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::Login
        }),
        fieldset![
            attrs! {
                        At::Disabled=> status.as_at_value(),
            },
            legend!["credentials"],
            label![attrs! { At::For => "username"}, "Username/Email"],
            input![
                id!("username"),
                attrs! {
                At::Required => true,
                At::Value=> model.credentials.target(),
                At::MinLength=> "5",
                At::Name => "username",
                At::MaxLength=> "25",
                At::Type=> "text"
                        },
                input_ev(Ev::Input, Msg::TargetChanged),
            ],
            label![attrs! { At::For => "password"}, "Password"],
            input![
                id!("password"),
                attrs! {
                    At::Required => true,
                    At::MinLength=> "8",
                    At::MaxLength=> "30"
                    At::Value => model.credentials.password(),
                    At::Name => "password",
                    At::Type=> "password"
                },
                input_ev(Ev::Input, Msg::PasswordChanged),
            ],
        ],
        button![
            "Login",
            attrs! {
            At::Type=> "submit"
                    },
        ],
        IF!(*status =>  div![C!["lds-ring"], div![], div![], div![], div![]] )
    ]
}
