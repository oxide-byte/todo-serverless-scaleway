use std::error::Error;
use reqwest::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, HeaderMap, HeaderValue};
use crate::models::Todo;

#[derive(Clone)]
pub struct TodoService {
    url_faas_get_all: String,
    url_faas_add: String,
    url_faas_edit: String,
    url_faas_delete: String
}

impl TodoService {

    pub fn new() -> TodoService {
        TodoService {
            url_faas_get_all: option_env!("URL_FAAS_GET_ALL").unwrap_or("http://localhost:8081/").to_string(),
            url_faas_add: option_env!("URL_FAAS_ADD").unwrap_or("http://localhost:8083/").to_string(),
            url_faas_edit: option_env!("URL_FAAS_EDIT").unwrap_or("http://localhost:8084/").to_string(),
            url_faas_delete: option_env!("URL_FAAS_DELETE").unwrap_or("http://localhost:8085/").to_string()
        }
    }

    pub async fn get_todos(&self) -> Result<Vec<Todo>, Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .get(self.url_faas_get_all.clone())
            .headers(construct_headers())
            //.fetch_mode_no_cors()
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        let todos: Vec<Todo> = response
            .json::<Vec<Todo>>()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(todos)
    }

    pub async fn delete_todo(&self, id: String) -> Result<(), Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .delete(format!("{}?id={}", self.url_faas_delete, id))
            .headers(construct_headers())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        Ok(())
    }

    pub async fn insert_todo(&self, todo:Todo) -> Result<(), Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .post(self.url_faas_add.clone())
            .headers(construct_headers())
            .json(&todo)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        Ok(())
    }

    pub async fn edit_todo(&self, todo:Todo) -> Result<(), Box<dyn Error>> {

        let client = reqwest::Client::new();

        let response = client
            .put(self.url_faas_edit.clone())
            .headers(construct_headers())
            .json(&todo)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to retrieve data",
            )));
        }

        Ok(())
    }
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    // For local tests add this header:
    // headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("access-control-allow-headers, access-control-allow-methods, access-control-allow-origin, access-control-allow-credentials, content-type"));
    headers.insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("PUT, GET, HEAD, POST, DELETE, OPTIONS"));
    headers.insert(ACCESS_CONTROL_ALLOW_CREDENTIALS,HeaderValue::from_static("true"));
    headers
}