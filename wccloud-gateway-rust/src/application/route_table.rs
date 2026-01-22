use crate::domain::models::{FilterConfig, PredicateConfig, RouteConfig};
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct RouteTable {
    routes: Arc<DashMap<String, RouteConfig>>,
}

impl RouteTable {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(DashMap::new()),
        }
    }

    pub fn add_route(&self, route: RouteConfig) {
        self.routes.insert(route.id.clone(), route);
    }

    pub fn get_route_by_path(&self, path: &str) -> Option<RouteConfig> {
        for route in self.routes.iter() {
            for predicate in &route.value().predicates {
                if path_matches_pattern(path, &predicate.path) {
                    return Some(route.value().clone());
                }
            }
        }
        None
    }

    pub fn update_route(&self, route: RouteConfig) {
        self.routes.insert(route.id.clone(), route);
    }

    pub fn remove_route(&self, id: &str) {
        self.routes.remove(id);
    }

    pub fn get_all_routes(&self) -> Vec<RouteConfig> {
        self.routes
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    // 从外部批量更新所有路由
    pub fn update_all_routes(&self, new_routes: Vec<RouteConfig>) {
        // 清除所有现有路由
        self.routes.clear();

        // 添加所有新路由
        for route in new_routes {
            self.routes.insert(route.id.clone(), route);
        }
    }
}

fn path_matches_pattern(path: &str, pattern: &str) -> bool {
    // Check if pattern contains wildcards
    if pattern.contains("**") || pattern.contains("*") {
        let escaped_pattern = regex::escape(pattern);
        let regex_pattern = escaped_pattern
            .replace("\\*\\*", ".*")
            .replace("\\*", "[^/]*");
        let full_pattern = format!("^{}$", regex_pattern);

        if let Ok(regex) = regex::Regex::new(&full_pattern) {
            regex.is_match(path)
        } else {
            false
        }
    } else {
        path == pattern
    }
}
