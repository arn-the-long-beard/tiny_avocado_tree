#![feature(map_into_keys_values)]

mod request;
use seed::{prelude::*, *};
extern crate heck;
use crate::{
    router::{ExtractedRoute, Router},
    theme::Theme,
    top_bar::TopBar,
};
use enum_map::Enum;
use heck::SnakeCase;
use shared::models::user::LoggedUser;

extern crate enum_map;

// use shared::User;
mod pages;
// pub mod route;
mod router;
mod theme;
mod top_bar;

const ADMIN: &str = "admin";
const LOGIN: &str = "login";
const REGISTER: &str = "register";
const DASHBOARD: &str = "dashboard";
// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .subscribe(Msg::UrlChanged)
        .subscribe(Msg::UrlRequested)
        .subscribe(Msg::UserLogged);
    let mut router: Router<Routes> = Router::new();
    router.set_base_url(url.to_base_url());
    router
        .add_route(Routes::Home, "home".to_string())
        .add_route(Routes::Register, "register".to_string())
        .add_route(Routes::Login, "login".to_string())
        .add_route(Routes::Dashboard, "dashboard".to_string());

    Model {
        theme: Theme::default(),
        state: Default::default(),
        router,
        logged_user: None,
    }
}

#[derive(Debug, Enum, Copy, Clone, PartialEq)]
pub enum Routes {
    Home,
    Login,
    Register,
    Dashboard,
    // Admin(page::admin::Model),
    NotFound,
}
// ------ ------
//     Model
// ------ ------

struct Model {
    state: State,
    router: Router<Routes>,
    logged_user: Option<LoggedUser>,
    theme: Theme,
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
// /// Here we manage the navigation depending of the url path
// impl Route {
//     fn init(mut url: Url) -> Self {
//         match url.next_path_part() {
//             None => Self::Home,
//             Some(LOGIN) => Self::Login,
//             Some(REGISTER) => Self::Register,
//             Some(DASHBOARD) => Self::Dashboard,
//             Some(_) => Self::NotFound,
//         }
//     }
//
//     fn is_active(&self, path: String) -> bool {
//         match &self {
//             Route::Home => path.eq("Home"),
//             Route::Login => path.eq("Login"),
//             Route::Register => path.eq("Register"),
//             Route::Dashboard => path.eq("Dashboard"),
//             Route::NotFound => path.eq("404"),
//         }
//     }
// }

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
/// Each component will have single action/message mapped to its message later
/// in update

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    UrlRequested(subs::UrlRequested),
    Register(pages::register::Msg),
    Login(pages::login::Msg),
    UserLogged(LoggedUser),
    SwitchToTheme(Theme),
}

/// Main update for the entire APP, every component action/message should me
/// mapped there because of single truth of path
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("URL has changed");

            // model.page = Route::init(url);
        }
        Msg::UrlRequested(request) => {
            log!("URL requested");
            let url = request.0;
            model.router.navigate_to_url(url);
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
            // orders.notify(subs::UrlRequested::new(
            //     Urls::new(&model.base_url).build_url(DASHBOARD),
            // ));
        }
        Msg::SwitchToTheme(theme) => model.theme = theme,
    }
}

// ------ ------
//     View
// ------ ------
/// View function which renders stuff to html
fn view(model: &Model) -> impl IntoNodes<Msg> {
    if model.logged_user.is_none() {
        vec![
            header(&model),
            if let Some(route) = &model.router.current_route {
                match route {
                    Routes::Home => home(&model.theme),
                    // Page::Admin(admin_model) => page::admin::view(admin_model, &model.ctx),
                    Routes::NotFound => div!["404"],
                    Routes::Login => pages::login::view(&model.state.login).map_msg(Msg::Login),
                    Routes::Register => {
                        pages::register::view(&model.state.register).map_msg(Msg::Register)
                    }
                    _ => div!["404"],
                }
            } else {
                home(&model.theme)
            },
        ]
    } else {
        vec![div!["Authenticated Routing not working"]]
        // vec![
        //     authenticated_header(&model.base_url, &model.page),
        //     match &model.page {
        //         Route::Dashboard => div![div!["Welcome Dashboard!"],],
        //         // Page::Admin(admin_model) => page::admin::view(admin_model,
        // &model.ctx),         Route::NotFound => div!["404"],
        //         _ => div!["404"],
        //     },
        // ]
    }
}

fn header(model: &Model) -> Node<Msg> {
    // let page = &model.page;
    // let base_url = &model.base_url;
    let mut list: Vec<Node<Msg>> = Vec::new();

    for route in &model.router.mapped_routes() {
        list.push(render_route(route));
    }
    div![
        TopBar::new("Welcome Guest").style(model.theme.clone()),
        ul![list]
    ]
}
//

fn render_route(route: &ExtractedRoute<Routes>) -> Node<Msg> {
    li![a![
        C!["route", IF!( route.is_active => "active-route" )],
        attrs! { At::Href => route.url },
        route.path.clone(),
    ]]
}
// // /// Render a route
// fn render_route(router : &Router<Routes>, route : Routes) -> Node<Msg> {
//     li![a![
//         C![
//             "route",
//             IF!(router. ) => "active-route" )
//         ],
//         attrs! { At::Href => Urls::new(base_url).build_url(path) },
//         path,
//     ]]
// }

// fn authenticated_header(base_url: &Url, page: &Route) -> Node<Msg> {
//     ul![route(base_url, page, "Dashboard"),]
// }

fn home(theme: &Theme) -> Node<Msg> {
    div![
        div!["Welcome home!"],
        match theme {
            Theme::Dark => {
                button![
                    "Switch to Light",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Light))
                ]
            }
            Theme::Light => {
                button![
                    "Switch to Dark",
                    ev(Ev::Click, |_| Msg::SwitchToTheme(Theme::Dark))
                ]
            }
        }
    ]
}
// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
