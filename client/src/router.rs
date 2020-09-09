use crate::Urls;
use enum_map::{Enum, EnumMap, Values};
use seed::Url;

// impl Clone for ExampleRoutes {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

pub struct Router<Routes: Enum<String> + Copy + Clone + PartialEq> {
    pub routes: EnumMap<Routes, String>,
    pub current_route: Option<Routes>,
    pub current_history_index: usize,
    base_url: Url,
    history: Vec<Routes>,
}

#[derive(Debug)]
pub struct ExtractedRoute<Routes: Enum<String> + Copy + Clone + PartialEq> {
    pub url: Url,
    pub is_active: bool,
    pub path: String,
    pub route: Routes,
}
impl<Routes: Enum<String> + Copy + Clone + PartialEq> Router<Routes> {
    pub fn new() -> Router<Routes> {
        Router {
            current_history_index: 0,
            routes: EnumMap::new(),
            history: Vec::new(),
            current_route: None,
            base_url: Url::new(), // should replace with current ,aybe ?
        }
    }
    pub fn set_base_url(&mut self, url: Url) -> &mut Self {
        self.base_url = url;
        self
    }

    pub fn routes_values(&'static self) -> Values<String> {
        let mut values = &self.routes.values();
        values.clone()
    }
    pub fn add_route(&mut self, route: Routes, value: String) -> &mut Self {
        self.routes[route] = value;
        self
    }

    fn push_to_history(&mut self, route: Routes) {
        self.history.push(route);
        self.current_history_index = self.history.len() - 1;
    }

    /// Go back in history and navigate back to the previous route
    ///  # Note for now it does not add to history since we navigate inside
    pub fn back(&mut self) -> bool {
        if let Some(next_route) = self.can_back_with_route() {
            self.current_route = Some(next_route);
            self.current_history_index -= 1;
            true
        } else {
            false
        }
    }

    /// Check if you can go back in history and give you the right route
    pub fn can_back_with_route(&self) -> Option<Routes> {
        // If we have no history, cannot go back

        if self.history.is_empty() {
            return None;
        }
        // If the current route is at index 0, we cannot go back more
        if self.current_history_index == 0 {
            return None;
        }
        let next_index = self.current_history_index - 1;
        let route = self.history.get(next_index).unwrap();
        Some(*route)
    }

    /// Check if you can navigate forward in the history
    pub fn can_forward_with_route(&self) -> Option<Routes> {
        // if there is no route, cannot go forward
        if self.history.is_empty() {
            return None;
        }
        // If we are on the last index, we cannot go forward neither
        if self.current_history_index == self.history.len() - 1 {
            return None;
        }
        let next_index = self.current_history_index + 1;

        let route = self.history.get(next_index).unwrap_or_else(|| {
            panic!(
                "We should have get route but index is failed {}",
                next_index
            )
        });
        Some(*route)
    }

    /// to move forward in the history
    /// # Note for now it does not add to history since we navigate inside
    pub fn forward(&mut self) -> bool {
        if let Some(next_route) = self.can_forward_with_route() {
            self.current_route = Some(next_route);
            self.current_history_index += 1;
            true
        } else {
            false
        }
    }

    pub fn is_current_route(&self, route: Routes) -> bool {
        if let Some(current_route) = self.current_route {
            route.eq(&current_route)
        } else {
            false
        }
    }

    fn reload_without_cache() {}

    /// Go to the next url with the associated route
    /// This will push to history. So If you go back multiple time and then use
    /// navigate and then go back, you will not get the previous page, but the
    /// one just pushed into history before
    pub fn navigate(&mut self, route: Routes) {
        self.current_route = Some(route);
        self.push_to_history(route);
    }

    pub fn navigate_to_url(&mut self, mut url: Url) {
        let path_result = url.next_path_part();
        if let Some(path) = path_result {
            if let Some(route_match) = self.mapped_routes().iter().find(|r| r.path == path) {
                self.navigate(route_match.route)
            }
        }
    }

    pub fn url(&self, route: Routes) -> Url {
        Urls::new(&self.base_url).build_url(&self.routes[route])
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn mapped_routes(&self) -> Vec<ExtractedRoute<Routes>> {
        let mut list: Vec<ExtractedRoute<Routes>> = Vec::new();
        for (route, path) in &self.routes {
            println!(" path ---> {:?}", path);
            let is_active = self.is_current_route(route);
            list.push(ExtractedRoute {
                url: self.url(route),
                path: path.to_string(),
                is_active,
                route,
            })
        }
        list
    }
}

#[cfg(test)]
mod test {
    use crate::router::Router;

    use enum_map::Enum;

    #[derive(Debug, Enum, Copy, Clone, PartialEq)]
    enum ExampleRoutes {
        Home,
        Login,
        Register,
        Stuff,
    }
    #[test]
    fn test_build_router() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home".to_string())
            .add_route(ExampleRoutes::Login, "login".to_string());

        assert_eq!(router.routes[ExampleRoutes::Home], "home");
        assert_eq!(router.routes[ExampleRoutes::Login], "login");
    }

    #[test]
    fn test_build_url() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home".to_string())
            .add_route(ExampleRoutes::Login, "login".to_string());

        let url = router.base_url().clone().add_path_part("home");
        let url_from_router = router.url(ExampleRoutes::Home);

        eprintln!("{:?}", url.path());
        eprintln!("{:?}", url_from_router.path());

        assert_eq!(url_from_router.path(), url.path());
    }
    #[test]
    fn test_navigation() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home".to_string())
            .add_route(ExampleRoutes::Login, "login".to_string());

        router.navigate(ExampleRoutes::Home);

        let is_home = matches!(router.current_route.unwrap(), ExampleRoutes::Home);

        assert_eq!(is_home, true);
        assert_eq!(router.current_history_index, 0);

        router.navigate(ExampleRoutes::Login);

        let is_home = matches!(router.current_route.unwrap(), ExampleRoutes::Home);

        assert_eq!(is_home, false);
        assert_eq!(router.current_history_index, 1);
    }
    #[test]
    fn test_backward() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home".to_string())
            .add_route(ExampleRoutes::Login, "login".to_string())
            .add_route(ExampleRoutes::Register, "register".to_string())
            .add_route(ExampleRoutes::Stuff, "stuff".to_string());

        let back = router.back();
        assert_eq!(back, false, "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index, 0,
            "We should have current index 0"
        );
        assert_eq!(
            router.current_route.is_none(),
            true,
            "We should not have current rou"
        );

        router.navigate(ExampleRoutes::Home);
        router.navigate(ExampleRoutes::Register);
        router.navigate(ExampleRoutes::Login);

        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Register);
        assert_eq!(router.is_current_route(ExampleRoutes::Register), true);
        let back = router.back();
        assert_eq!(back, true, "We should have gone backwards");
        assert_eq!(router.current_history_index, 0);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Home);
        assert_eq!(router.is_current_route(ExampleRoutes::Home), true);
        router.navigate(ExampleRoutes::Stuff);
        println!("{:?}", router.current_history_index);
        let back = router.back();
        assert_eq!(back, true);
        // Here is tricky part, after navigate we go back to the end of history, so if
        // we go back, we go to the previous index
        assert_eq!(router.current_history_index, 2);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Login);
    }

    #[test]
    fn test_forward() {
        let mut router: Router<ExampleRoutes> = Router::new();

        router
            .add_route(ExampleRoutes::Home, "home".to_string())
            .add_route(ExampleRoutes::Login, "login".to_string())
            .add_route(ExampleRoutes::Register, "register".to_string())
            .add_route(ExampleRoutes::Stuff, "stuff".to_string());

        let back = router.back();
        assert_eq!(back, false, "We should Not have gone backwards");
        assert_eq!(
            router.current_history_index, 0,
            "We should have current index 0"
        );
        assert_eq!(
            router.current_route.is_none(),
            true,
            "We should not have current rou"
        );

        router.navigate(ExampleRoutes::Home);
        router.navigate(ExampleRoutes::Register);
        router.navigate(ExampleRoutes::Login);

        assert_eq!(router.current_history_index, 2);

        let back = router.back();
        let back = router.back();

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 1);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Register);

        let forward = router.forward();
        assert_eq!(forward, true, "We should have gone forward");
        assert_eq!(router.current_history_index, 2);
        assert_eq!(router.current_route.unwrap(), ExampleRoutes::Login);
        let forward = router.forward();
        assert_eq!(forward, false, "We should Not have gone forward");
    }
}
