// src/todo_api.rs
use actix_web::{web, App, HttpServer, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

// Data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

// Request DTOs (Data Transfer Objects)
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

// Application state - thread-safe shared storage
type TodoStorage = Arc<Mutex<HashMap<Uuid, TodoItem>>>;

// API Handlers

// GET /todos - List all todos
async fn get_todos(data: web::Data<TodoStorage>) -> Result<impl Responder> {
    let todos = data.lock().unwrap();
    let todo_list: Vec<&TodoItem> = todos.values().collect();
    Ok(HttpResponse::Ok().json(todo_list))
}

// GET /todos/{id} - Get a specific todo
async fn get_todo(
    path: web::Path<Uuid>,
    data: web::Data<TodoStorage>,
) -> Result<impl Responder> {
    let todo_id = path.into_inner();
    let todos = data.lock().unwrap();

    match todos.get(&todo_id) {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        }))),
    }
}

// POST /todos - Create a new todo
async fn create_todo(
    req: web::Json<CreateTodoRequest>,
    data: web::Data<TodoStorage>,
) -> Result<impl Responder> {
    let mut todos = data.lock().unwrap();

    let new_todo = TodoItem {
        id: Uuid::new_v4(),
        title: req.title.clone(),
        completed: false,
    };

    todos.insert(new_todo.id, new_todo.clone());

    Ok(HttpResponse::Created().json(new_todo))
}

// PUT /todos/{id} - Update an existing todo
async fn update_todo(
    path: web::Path<Uuid>,
    req: web::Json<UpdateTodoRequest>,
    data: web::Data<TodoStorage>,
) -> Result<impl Responder> {
    let todo_id = path.into_inner();
    let mut todos = data.lock().unwrap();

    match todos.get_mut(&todo_id) {
        Some(todo) => {
            if let Some(title) = &req.title {
                todo.title = title.clone();
            }
            if let Some(completed) = req.completed {
                todo.completed = completed;
            }
            Ok(HttpResponse::Ok().json(todo.clone()))
        }
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        }))),
    }
}

// DELETE /todos/{id} - Delete a todo
async fn delete_todo(
    path: web::Path<Uuid>,
    data: web::Data<TodoStorage>,
) -> Result<impl Responder> {
    let todo_id = path.into_inner();
    let mut todos = data.lock().unwrap();

    match todos.remove(&todo_id) {
        Some(_) => Ok(HttpResponse::NoContent().finish()),
        None => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "Todo not found"
        }))),
    }
}

// Server startup function
pub async fn start_todo_api() -> std::io::Result<()> {
    println!("🚀 Starting Todo API server...");

    // Initialize shared state
    let todo_storage: TodoStorage = Arc::new(Mutex::new(HashMap::new()));

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(todo_storage.clone()))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
        .bind("127.0.0.1:8081")? // Using 8081 to avoid conflict with your ID service
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_create_todo() {
        let todo_storage: TodoStorage = Arc::new(Mutex::new(HashMap::new()));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(todo_storage))
                .route("/todos", web::post().to(create_todo))
        ).await;

        let req_body = CreateTodoRequest {
            title: "Test todo".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_todos_empty() {
        let todo_storage: TodoStorage = Arc::new(Mutex::new(HashMap::new()));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(todo_storage))
                .route("/todos", web::get().to(get_todos))
        ).await;

        let req = test::TestRequest::get().uri("/todos").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}