use seed::{prelude::*, *};
use shared::models::user::User;
#[derive(Default)]
pub struct Model {
    user: User,
    password_power: Power,
}

pub enum Power {
    NoPower,
    BabyPower(u8),
    LowPower(u8),
    MediumPower(u8),
    HighPower(u8),
    MaxPower(u8),
}
impl Default for Power {
    fn default() -> Self {
        Power::NoPower
    }
}
impl Power {
    pub fn rank(nb: u8) -> Power {
        match nb {
            0 => Power::NoPower,
            1..=10 => Power::BabyPower(nb),
            11..=25 => Power::LowPower(nb),
            26..=100 => Power::MediumPower(nb),
            101..=200 => Power::HighPower(nb),
            201..=255 => Power::MaxPower(nb),
            _ => Power::NoPower,
        }
    }
    pub fn display(&self) -> &str {
        match &self {
            Power::NoPower => "Cannot live",
            Power::BabyPower(pw) => "I am a baby and I cry a lot",
            Power::LowPower(pw) => "I am so weak ",
            Power::MediumPower(pw) => "I am ok ",
            Power::HighPower(pw) => "I feel so good ",
            Power::MaxPower(pw) => "Mouahaha I am so OP",
        }
    }
    pub fn class(&self) -> &str {
        match &self {
            Power::NoPower => "no-power",
            Power::BabyPower(pw) => "baby-power",
            Power::LowPower(pw) => "low-power",
            Power::MediumPower(pw) => "medium-power",
            Power::HighPower(pw) => "high-power",
            Power::MaxPower(pw) => "max-power",
        }
    }
    pub fn units(&self) -> &u8 {
        match &self {
            Power::NoPower => &0,
            Power::BabyPower(pw) => pw,
            Power::LowPower(pw) => pw,
            Power::MediumPower(pw) => pw,
            Power::HighPower(pw) => pw,
            Power::MaxPower(pw) => pw,
        }
    }
}
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
        Msg::PasswordChanged(text) => {
            let text = text.trim();
            let mut password_power = 0;
            model.user.credentials.set_password(text.to_string());

            let pwd = model.user.credentials.password();
            let characters = pwd.chars();
            let compared_characteres = characters.clone();
            password_power = characters.clone().count() as u8;
            for c in characters {
                if c.is_numeric() {
                    password_power += 1;
                }
                if c.is_uppercase() {
                    password_power += 1;
                }
                if c.is_ascii_punctuation() {
                    password_power += 2;
                }
                let count = compared_characteres.clone().filter(|o| c.eq(o)).count();
                if count == 1 {
                    password_power += 4;
                } else if (count > 1) & (count < 3) {
                    password_power += 2;
                } else {
                    password_power -= 1;
                }
            }
            model.password_power = Power::rank(password_power)
        }
        Msg::UsernameChanged(text) => model.user.credentials.set_username(text.trim().to_string()),
        Msg::FirstNameChanged(text) => model.user.first_name = text.trim().to_string(),
        Msg::LastNameChanged(text) => model.user.last_name = text.trim().to_string(),
        Msg::EmailChanged(text) => {
            model.user.credentials.set_email(text.trim().to_string());
        }
    }
}

/// view of register page
pub fn view(model: &Model) -> Node<Msg> {
    let user = &model.user;
    let power = &model.password_power;

    div![form![
        attrs! {
        At::Action => "/api/register",
        At::Method=> "post",
        At::Target=> "_top"
        },
        fieldset![
            legend!["credentials"],
            label![attrs! { At::For => "username"}, "Username"],
            input![
                id!("username"),
                attrs! {
                At::Required => true,
                At::Value=> user.credentials.username(),
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
                At::Value => user.credentials.email(),
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
        input![attrs! {
            At::MaxLength=> "15"
            At::Type=> "submit" ,
            At::Value=> "Register" ,
        }],
    ]]
}
