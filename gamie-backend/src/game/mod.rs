mod models;
mod routes;
mod state;
mod ws;

pub use models::{Room, Player, GameState as GameStateModel, GameStatus, Move};
pub use routes::game_routes;
pub use state::GameState;
pub use ws::{game_ws, GameMessage};

use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub creator_id: String,
}

pub async fn create_room(
    req: web::Json<CreateRoomRequest>,
    state: web::Data<GameState>,
) -> Result<HttpResponse, crate::error::Error> {
    let room_id = Uuid::new_v4().to_string();
    let room = state
        .create_room(room_id, req.name.clone(), req.creator_id.clone())
        .await?;
    
    Ok(HttpResponse::Ok().json(room))
}

pub async fn get_room(
    room_id: web::Path<String>,
    state: web::Data<GameState>,
) -> Result<HttpResponse, crate::error::Error> {
    let room = state.get_room(&room_id).await?;
    Ok(HttpResponse::Ok().json(room))
}

pub fn game_routes() -> Scope {
    web::scope("/game")
        .app_data(web::Data::new(GameState::new()))
        .route("/ws", web::get().to(game_ws))
        .route("/rooms", web::post().to(create_room))
        .route("/rooms/{room_id}", web::get().to(get_room))
} 