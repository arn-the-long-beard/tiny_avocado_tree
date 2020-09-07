mod request;
use seed::{prelude::*, *};
extern crate heck;
use heck::SnakeCase;
use shared::models::user::LoggedUser;

// use shared::User;
mod pages;

const ADMIN: &str = "admin";
const LOGIN: &str = "login";
const REGISTER: &str = "register";
const DASHBOARD: &str = "dashboard";
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged).subscribe(Msg::UserLogged);

    Model {
        state: Default::default(),
        base_url: url.to_base_url(),
        page: Route::init(url),
        logged_user: None,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    state: State,
    base_url: Url,
    logged_user: Option<LoggedUser>,
    page: Route,
}

// ------ State for component ------
#[derive(Default)]
pub struct State {
    pub register: pages::register::Model,
    pub login: pages::login::Model,
}

/// Page Struct regroups all the possible root navigate on your App
/// It is possible to define nested navigation ex : see seed/example/pages
enum Route {
    Home,
    Login,
    Register,
    Dashboard,
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
            Some(DASHBOARD) => Self::Dashboard,
            Some(_) => Self::NotFound,
        }
    }

    fn is_active(&self, path: String) -> bool {
        match &self {
            Route::Home => path.eq("Home"),
            Route::Login => path.eq("Login"),
            Route::Register => path.eq("Register"),
            Route::Dashboard => path.eq("Dashboard"),
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
///

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Register(pages::register::Msg),
    Login(pages::login::Msg),
    UserLogged(LoggedUser),
}

/// Main update for the entire APP, every component action/message should me mapped there because of single truth of path
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Route::init(url);
        }
        Msg::Register(register_message) => pages::register::update(
            register_message,
            &mut model.state.register,
            &mut orders.proxy(Msg::Register),
        ),
        Msg::Login(login_message) => pages::login::update(
            login_message,
            &mut model.state.login,
            &mut orders.proxy(Msg::Login),
        ),
        Msg::UserLogged(user) => {
            log!("got user logged");
            model.logged_user = Some(user);
            orders.notify(subs::UrlRequested::new(
                Urls::new(&model.base_url).build_url(DASHBOARD),
            ));
        }
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    if model.logged_user.is_none() {
        vec![
            header(&model.base_url, &model.page),
            match &model.page {
                Route::Home => div![div!["Welcome home!"],],
                // Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
                Route::NotFound => div!["404"],
                Route::Login => pages::login::view(&model.state.login).map_msg(Msg::Login),
                Route::Register => {
                    pages::register::view(&model.state.register).map_msg(Msg::Register)
                }
                _ => div!["404"],
            },
        ]
    } else {
        vec![
            authenticated_header(&model.base_url, &model.page),
            match &model.page {
                Route::Dashboard => div![div!["Welcome Dashboard!"],],
                // Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
                Route::NotFound => div!["404"],
                _ => div!["404"],
            },
        ]
    }
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

fn authenticated_header(base_url: &Url, page: &Route) -> Node<Msg> {
    ul![route(base_url, page, "Dashboard"),]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
