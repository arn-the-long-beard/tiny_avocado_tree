#![allow(clippy::must_use_candidate)]

use seed::{prelude::*, *};
extern crate heck;
use heck::SnakeCase;
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
    pub logged_user: pages::register::Model,
    pub register_data: pages::register::Model,
    pub auth_data: pages::login::Model,
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
///
impl Route {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(LOGIN) => Self::Login,
            Some(REGISTER) => Self::Register,
            Some(_) => Self::NotFound,
        }
    }

    fn is_active(&self, path: String) -> bool {
        match &self {
            Route::Home => path.eq("Home"),
            Route::Login => path.eq("Login"),
            Route::Register => path.eq("Register"),
            Route::NotFound => path.eq("404"),
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
/// Construct url injected in the web browser with path
impl<'a> Urls<'a> {
    pub fn build_url(self, path: &str) -> Url {
        if path.eq("Home") {
            self.base_url()
        } else {
            self.base_url().add_path_part(path.to_snake_case())
        }
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
        header(&model.base_url, &model.page),
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

fn header(base_url: &Url, page: &Route) -> Node<Msg> {
    ul![
        route(base_url, page, "Home"),
        route(base_url, page, "Login"),
        route(base_url, page, "Register"),
    ]
}
/// Render a route
fn route(base_url: &Url, page: &Route, path: &str) -> Node<Msg> {
    li![a![
        C![
            "route",
            IF!(page.is_active( path.to_string() ) => "active-route" )
        ],
        attrs! { At::Href => Urls::new(base_url).build_url(path) },
        path,
    ]]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
