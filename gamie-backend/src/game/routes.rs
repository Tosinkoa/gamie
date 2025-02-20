use super::{ws::game_ws, GameState};
use actix_web::web;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub creator_id: String,
}

async fn create_room(
    req: web::Json<CreateRoomRequest>,
    state: web::Data<GameState>,
) -> Result<HttpResponse, crate::error::Error> {
    let room_id = Uuid::new_v4().to_string();
    let room = state
        .create_room(room_id, req.name.clone(), req.creator_id.clone())
        .await?;
    
    Ok(HttpResponse::Ok().json(room))
}

async fn get_room(
    room_id: web::Path<String>,
    state: web::Data<GameState>,
) -> Result<HttpResponse, crate::error::Error> {
    let room = state.get_room(&room_id).await?;
    Ok(HttpResponse::Ok().json(room))
}

pub fn game_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuring game routes");
    
    cfg.service(
        web::scope("/game")
            .app_data(web::Data::new(GameState::new()))
            // WebSocket endpoint
            .route("/ws", web::get().to(game_ws))
            // Room management endpoints
            .service(
                web::resource("/rooms")
                    .route(web::post().to(create_room))
            )
            .service(
                web::resource("/rooms/{room_id}")
                    .route(web::get().to(get_room))
            ),
    );
    
    info!("Game routes configured");
} 