use seed::{prelude::*, *};
use shared::models::power::Power;
use shared::models::user::User;

#[derive(Default)]
pub struct Model {
    user: User,
    password_power: Power,
    request_state: RequestState<User>,
}

pub enum RequestState<T> {
    Success(T),
    Failed { message: String, code: String },
    IsPending(bool),
}

impl<T> Default for RequestState<T> {
    fn default() -> Self {
        RequestState::IsPending(false)
    }
}
/// Action on register page
pub enum Msg {
    Register,
    RegisterFailed { message: String, code: String },
    RegisterSucceed(User),
    PasswordChanged(String),
    UsernameChanged(String),
    EmailChanged(String),
    FirstNameChanged(String),
    LastNameChanged(String),
    //todo implement a clear for the form ?
    Clear,
}
/// Update on register pages
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Register => {
            model.request_state = RequestState::IsPending(true);
            let request = Request::new("/api/register")
                .method(Method::Post)
                .json(&model.user)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    Msg::RegisterSucceed(response.json().await.unwrap())
                } else {
                    Msg::RegisterFailed {
                        message: response.text().await.unwrap(),
                        code: response.status().code.to_string(),
                    }
                }
            });
        }
        Msg::Clear => {}
        Msg::PasswordChanged(text) => {
            let text = text.trim();
            model.user.credentials.set_password(text.to_string());
            model.password_power = Power::rank(Power::calculate_power(
                model.user.credentials.password().to_string(),
            ));
        }
        Msg::UsernameChanged(text) => model.user.credentials.set_username(text.trim().to_string()),
        Msg::FirstNameChanged(text) => model.user.first_name = text.trim().to_string(),
        Msg::LastNameChanged(text) => model.user.last_name = text.trim().to_string(),
        Msg::EmailChanged(text) => {
            model.user.credentials.set_email(text.trim().to_string());
        }
        Msg::RegisterFailed { message, code } => {
            model.request_state = RequestState::Failed { message, code }
        }
        Msg::RegisterSucceed(user) => model.request_state = RequestState::Success(user),
    }
}

/// view of register page
pub fn view(model: &Model) -> Node<Msg> {
    match &model.request_state {
        RequestState::Success(user) => div![
            C!["welcome"],
            p![format!(
                "Thank you for your registration {} {}. :)",
                user.first_name, user.last_name
            )],
            br![],
            p![
                span!["You can now "],
                a![attrs! { At::Href => "./login" }, "login",],
                span![" as ",],
                span![
                    style! {St::Color => "darkblue"},
                    user.credentials.username(),
                    "."
                ]
            ]
        ],
        RequestState::IsPending(status) => form(model, status),
        RequestState::Failed { message, code } => p![format!(
            "En error happened {} with the code {}",
            message, code
        )],
    }
}

fn form(model: &Model, status: &bool) -> Node<Msg> {
    let user = &model.user;
    let power = &model.password_power;
    form![
        ev(Ev::Submit, |event| {
            event.prevent_default();
            Msg::Register
        }),
        fieldset![
            attrs! {
                        At::Disabled=> status.as_at_value(),
            },
            legend!["credentials"],
            label![attrs! { At::For => "username"}, "Username"],
            input![
                id!("username"),
                attrs! {
                At::Required => true,
                At::Value=> user.credentials.username(),
                At::MinLength=> "5",
                At::Name => "username",
                At::MaxLength=> "25",
                At::Type=> "text"
                        },
                input_ev(Ev::Input, Msg::UsernameChanged),
            ],
            label![attrs! { At::For => "email"}, "Email"],
            input![
                id!("email"),
                attrs! {
                At::Required => true,
                At::Value => user.credentials.email(),
                At::MinLength=> "5",
                At::MaxLength=> "25"
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
                    At::MaxLength=> "30"
                    At::Value => user.credentials.password(),
                    At::Name => "password",
                    At::Type=> "password"
                },
                input_ev(Ev::Input, Msg::PasswordChanged),
            ],
            p![format!("Password power => {} ", power.display())],
            div![
                C![power.class(), "power"],
                IF!(power.units().eq(&0) => i![C!["fas fa-skull-crossbones tiny−avocado-icons", power.class()]]),
                style! {St::Width => format!("{}px", power.units())
                },
            ]
        ],
        fieldset![
            attrs! {
                        At::Disabled=> status.as_at_value(),
            },
            legend!["Personal information"],
            label![attrs! { At::For => "first_name"}, "First Name"],
            input![
                id!("first_name"),
                attrs! {
                At::Required => true,
                At::Name => "first_name",
                At::Type=> "text",
                At::Value=> user.first_name,
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
                At::Value=> user.last_name,
                       },
                input_ev(Ev::Input, Msg::LastNameChanged),
            ],
            br![],
        ],
        button![
            "Register",
            attrs! {
            At::Disabled=> (power.units()<&101 && !(*status)).as_at_value(),
            At::Type=> "submit"
                    },
        ],
        IF!(*status =>  div![C!["lds-ring"], div![], div![], div![], div![]] )
    ]
}
