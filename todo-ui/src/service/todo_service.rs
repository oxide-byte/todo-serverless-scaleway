use std::error::Error;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
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
    headers
}