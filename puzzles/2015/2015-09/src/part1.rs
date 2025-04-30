use crate::router::{Route, Router};

pub fn part_1(input: &str) -> usize {
    let routes = input.trim().lines().map(|line| {
        let route: Route = line.trim().parse().unwrap();
        route
    });

    let router = Router::build_tree(routes);

    *router.paths().iter().min().unwrap()
}
