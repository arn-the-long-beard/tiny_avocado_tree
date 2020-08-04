#![allow(clippy::must_use_candidate)]

use seed::{prelude::*, *};
use shared::models::auth::AuthData;
use shared::models::user::{LoggedUser, User};

// use shared::User;
mod pages;

const ADMIN: &str = "admin";
const LOGIN: &str = "login";
const REGISTER: &str = "register";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        state: Default::default(),
        base_url: url.to_base_url(),
        page: Route::init(url),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    state: State,
    base_url: Url,
    page: Route,
}

// ------ State for component ------
#[derive(Default)]
pub struct State {
    pub logged_user: Option<LoggedUser>,
    pub register_data: Option<User>,
    pub auth_data: Option<AuthData>,
}

/// Page Struct regroups all the possible root navigate on your App
/// It is possible to define nested navigation ex : see seed/example/pages
enum Route {
    Home,
    Login,
    Register,
    // Admin(page::admin::Model),
    NotFound,
}
/// Here we manage the navigation depending of the url path
impl Route {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(LOGIN) => Self::Login,
            Some(REGISTER) => Self::Register,
            Some(_) => Self::NotFound,
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
/// Construct url injected in the web browser with path
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN)
    }
    pub fn register(self) -> Url {
        self.base_url().add_path_part(REGISTER)
    }
}

// ------ ------
//    Update
// ------ ------
/// Root actions for your app.
/// Each component will have single action/message mapped to its message later in update
enum Msg {
    UrlChanged(subs::UrlChanged),
    Register(pages::register::Msg),
    Login(pages::login::Msg),
}

/// Main update for the entire APP, every component action/message should me mapped there because of single truth of path
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Route::init(url);
        }
        Msg::Register(register_message) => pages::register::update(
            register_message,
            &mut model.state.register_data,
            &mut orders.proxy(Msg::Register),
        ),
        Msg::Login(_) => {}
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model.base_url),
        match &model.page {
            Route::Home => div![div!["Welcome home!"],],
            // Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
            Route::NotFound => div!["404"],
            Route::Login => pages::login::view(&model.state.auth_data).map_msg(Msg::Login),
            Route::Register => {
                pages::register::view(&model.state.register_data).map_msg(Msg::Register)
            }
        },
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    ul![
        li![a![
            attrs! { At::Href => Urls::new(base_url).home() },
            "Home",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).login() },
            "Login",
        ]],
        li![a![
            attrs! { At::Href => Urls::new(base_url).register() },
            "Register",
        ]],
        // li![a![
        //     attrs! { At::Href => Urls::new(base_url).admin_urls().report_urls().default() },
        //     "Report",
        // ]],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
