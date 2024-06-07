use actix_web::{App, delete, get, HttpResponse, HttpServer, post, put, Responder, web};
use actix_web::web::{Path, Query};
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder};
use sea_orm::prelude::Uuid;
use serde::Deserialize;
use serde_json::json;

use entities::address;
use migration::MigratorTrait;

#[derive(Debug, Clone)]
struct AppState {
    connection: DatabaseConnection,
}

#[derive(Deserialize)]
struct ParamPaginationAddress {
    page: u64,
    per_page: u64,
}

#[derive(Deserialize)]
struct AddressModelPutDto {
    postal_code: String,
    city: String,
}

#[derive(Deserialize)]
struct AddressModelCreateDto {
    postal_code: String,
    city: String,
}

#[get("/address/{id}")]
async fn get_address_by_id(state: web::Data<AppState>, query_param: Path<Uuid>) -> impl Responder {
    let result = address::Entity::find_by_id(query_param.into_inner()).one(&state.connection).await.unwrap();
    match result {
        None => HttpResponse::NotFound().body("not found"),
        Some(m) => HttpResponse::Ok().json(m)
    }
}

#[get("/addresses")]
async fn get_all_addresses(state: web::Data<AppState>, query_param: Query<ParamPaginationAddress>) -> impl Responder {
    let paginator = address::Entity::find()
        .order_by_asc(address::Column::Id)
        .paginate(&state.connection, query_param.per_page);
    let num_page = paginator.num_pages().await.unwrap();
    let address = paginator.fetch_page(query_param.page - 1).await.unwrap();

    let response_body = serde_json::to_string(&json!({
        "address" : address,
        "num_page" : num_page
    })).unwrap();
    HttpResponse::Ok().body(response_body)
}

#[put("/update-address/{id}")]
async fn update_address(state: web::Data<AppState>, query_param: Path<Uuid>, payload: web::Payload) -> impl Responder {
    let body_bytes = payload.to_bytes().await.unwrap();
    let body: AddressModelPutDto = serde_json::from_str(std::str::from_utf8(&body_bytes).unwrap()).unwrap();
    let result = address::Entity::find_by_id(query_param.into_inner()).one(&state.connection).await.unwrap();
    match result {
        None => HttpResponse::NotFound().body("not found"),
        Some(m) => {
            let mut m: address::ActiveModel = m.into();
            m.city = ActiveValue::Set(body.city);
            m.postal_code = ActiveValue::Set(body.postal_code);
            let new = m.update(&state.connection).await.unwrap();
            HttpResponse::Ok().json(serde_json::to_value(new).unwrap())
        }
    }
}

#[delete("/delete-address/{id}")]
async fn delete_address(state: web::Data<AppState>, query_param: Path<Uuid>) -> impl Responder {
    let r = address::Entity::delete_by_id(query_param.into_inner()).exec(&state.connection).await.unwrap();
    match r.rows_affected {
        0 => HttpResponse::NotFound().json(json!({
            "value" : "address not found"
        })),
        _ => HttpResponse::Ok().json(json!({
            "value" : "address deleted"
        }))
    }
}

#[post("/create-address")]
async fn create_address(state: web::Data<AppState>, payload: web::Payload) -> impl Responder {
    let body_bytes = payload.to_bytes().await.unwrap();
    let body: AddressModelCreateDto = serde_json::from_str(std::str::from_utf8(&body_bytes).unwrap()).unwrap();
    let new_id = Uuid::new_v4();
    let new_address = address::ActiveModel {
        id: ActiveValue::Set(new_id),
        postal_code: ActiveValue::Set(body.postal_code),
        city: ActiveValue::Set(body.city),
    };
    let result = address::Entity::insert(new_address).exec(&state.connection).await;

    match result {
        Ok(_) => {
            let new_model = address::Entity::find_by_id(new_id).one(&state.connection).await.unwrap();
            HttpResponse::Ok().json(new_model)
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(json!({
                "value" : "error during database insertion"
            }))
        }
    }
}

pub async fn create() -> std::io::Result<()> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL introuvable !");
    let db = Database::connect(url).await.unwrap();

    migration::Migrator::fresh(&db).await.unwrap();
    HttpServer::new(move || {
        App::new().service(
            get_all_addresses
        ).service(get_address_by_id)
            .service(update_address)
            .service(delete_address)
            .service(create_address)
            .app_data(web::Data::new(AppState {
                connection: db.clone(),
            }))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;
    Ok(())
}
