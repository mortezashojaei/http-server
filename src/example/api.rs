use crate::http::{Method, Request, Response, StatusCode};
use crate::server::Handler;
use std::fs;

pub struct ApiHandler {
    items: Vec<String>,
}

impl ApiHandler {
    pub fn new(path: &str) -> Self {
        let items = fs::read_to_string(path)
            .unwrap_or_else(|_| String::new())
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect();

        ApiHandler { items }
    }

    fn list_items(&self, request: &Request) -> Response {
        let query = request.query_string();
        let search = query
            .and_then(|qs| qs.get("q"))
            .map(str::to_lowercase);
        let page = query
            .and_then(|qs| qs.get("page"))
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(1);
        let limit = query
            .and_then(|qs| qs.get("limit"))
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|value| *value > 0)
            .unwrap_or(10);

        let filtered: Vec<&String> = self
            .items
            .iter()
            .filter(|item| match &search {
                Some(q) => item.to_lowercase().contains(q),
                None => true,
            })
            .collect();

        let total = filtered.len();
        let start = (page - 1).saturating_mul(limit);
        let page_items = filtered.into_iter().skip(start).take(limit);

        let items_json = page_items
            .map(|item| format!("\"{}\"", escape_json(item)))
            .collect::<Vec<_>>()
            .join(",");

        let body = format!(
            "{{\"items\":[{}],\"page\":{},\"limit\":{},\"total\":{}}}",
            items_json, page, limit, total
        );

        Response::json(StatusCode::Ok, body)
    }

    fn add_item(&mut self, request: &Request) -> Response {
        let item = match request.body_text() {
            None => {
                return Response::new(StatusCode::BadRequest, Some("Missing body".to_string()));
            }
            Some(Err(_)) => {
                return Response::new(
                    StatusCode::BadRequest,
                    Some("Invalid UTF-8 body".to_string()),
                );
            }
            Some(Ok(text)) => text.trim(),
        };

        if item.is_empty() {
            return Response::new(StatusCode::BadRequest, Some("Empty body".to_string()));
        }

        self.items.push(item.to_string());

        Response::json(
            StatusCode::Ok,
            format!(
                "{{\"item\":\"{}\",\"total\":{}}}",
                escape_json(item),
                self.items.len()
            ),
        )
    }
}

impl Handler for ApiHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match (request.method(), request.path()) {
            (Method::GET, "/") => Response::json(
                StatusCode::Ok,
                "{\"message\":\"API ready\",\"endpoints\":[\"GET /items?q=&page=1&limit=10\",\"POST /items\"]}"
                    .to_string(),
            ),
            (Method::GET, "/items") => self.list_items(request),
            (Method::POST, "/items") => self.add_item(request),
            _ => Response::new(StatusCode::NotFound, Some("Not Found".to_string())),
        }
    }
}

fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
