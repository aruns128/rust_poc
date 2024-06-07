use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use log::{info, warn};

// Data model for example purposes
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    name: String,
}

// In-memory storage
#[derive(Debug)]
pub struct AppState {
    pub(crate) items: Arc<Mutex<Vec<Item>>>,
}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let items: &Vec<Item> = &*data.items.lock().unwrap();
    HttpResponse::Ok().json(items)
}

pub async fn create(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items: std::sync::MutexGuard<'_, Vec<Item>> = data.items.lock().unwrap();
    items.push(item.into_inner());
    info!("Item created successfully");
    HttpResponse::Created().json("Item created successfully")
}

pub async fn update(item: web::Json<Item>, data: web::Data<AppState>) -> impl Responder {
    let mut items: std::sync::MutexGuard<'_, Vec<Item>> = data.items.lock().unwrap();
    let updated_item: Item = item.into_inner();
    let index: Option<usize> = items.iter().position(|i: &Item| i.id == updated_item.id);
    match index {
        Some(i) => {
            items[i] = updated_item;
            info!("Item updated successfully");
            HttpResponse::Ok().json("Item updated successfully")
        }
        None => {
            warn!("Item not found");
            HttpResponse::NotFound().json("Item not found")
        },
    }
}

pub async fn delete(item_id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let mut items: std::sync::MutexGuard<'_, Vec<Item>> = data.items.lock().unwrap();
    let id: u32 = item_id.into_inner();
    let index: Option<usize> = items.iter().position(|i: &Item| i.id == id);
    match index {
        Some(i) => {
            items.remove(i);
            info!("Item deleted successfully");
            HttpResponse::Ok().json("Item deleted successfully")
        }
        None => {
            warn!("Item not found");
            HttpResponse::NotFound().json("Item not found")
        },
    }
}

pub async fn rest_crud() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        items: Arc::new(Mutex::new(Vec::new())),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index)) // Route for "/"
            .route("/create", web::post().to(create)) // Route for "/create"
            .route("/update", web::put().to(update)) // Route for "/update"
            .route("/delete/{id}", web::delete().to(delete)) // Route for "/delete"
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
