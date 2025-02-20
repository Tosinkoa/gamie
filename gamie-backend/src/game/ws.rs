use actix_ws::Message;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::time::Duration;
use chrono::Utc;

use crate::game::models::{Room, GameStatus, Move};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum GameMessage {
    Join {
        room_id: String,
        player_id: String,
        username: String,
    },
    Leave {
        room_id: String,
        player_id: String,
    },
    StartGame {
        room_id: String,
        player_id: String,
    },
    MakeMove {
        room_id: String,
        player_id: String,
        number: i32,
        position: usize,
    },
    GameState {
        room: Room,
    },
    Error {
        message: String,
    },
}

pub struct GameSession {
    room_id: Option<String>,
    player_id: Option<String>,
    heartbeat: Instant,
    tx: mpsc::Sender<Message>,
}

impl GameSession {
    pub fn new(tx: mpsc::Sender<Message>) -> Self {
        Self {
            room_id: None,
            player_id: None,
            heartbeat: Instant::now(),
            tx,
        }
    }

    pub async fn handle_message(&mut self, msg: Message, state: &GameState) -> Result<(), Error> {
        match msg {
            Message::Text(text) => {
                let game_msg: GameMessage = serde_json::from_str(&text)?;
                self.handle_game_message(game_msg, state).await?;
            }
            Message::Close(reason) => {
                if let Some(room_id) = &self.room_id {
                    if let Some(player_id) = &self.player_id {
                        self.handle_game_message(
                            GameMessage::Leave {
                                room_id: room_id.clone(),
                                player_id: player_id.clone(),
                            },
                            state,
                        )
                        .await?;
                    }
                }
            }
            Message::Ping(bytes) => {
                self.heartbeat = Instant::now();
                self.tx.send(Message::Pong(bytes)).await?;
            }
            Message::Pong(_) => {
                self.heartbeat = Instant::now();
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_game_message(&mut self, msg: GameMessage, state: &GameState) -> Result<(), Error> {
        match msg {
            GameMessage::Join { room_id, player_id, username } => {
                let mut room = state.get_room(&room_id).await?;
                if room.add_player(player_id.clone(), username) {
                    self.room_id = Some(room_id);
                    self.player_id = Some(player_id);
                    state.broadcast_room_state(&room).await?;
                } else {
                    self.tx.send(Message::Text(serde_json::to_string(&GameMessage::Error {
                        message: "Cannot join room".to_string(),
                    })?)).await?;
                }
            }
            GameMessage::StartGame { room_id, player_id } => {
                let mut room = state.get_room(&room_id).await?;
                if room.can_start() && room.creator_id == player_id {
                    room.game_state.status = GameStatus::Starting;
                    room.game_state.generate_sequence(10); // Generate a sequence of 10 numbers
                    room.game_state.start_time = Some(Utc::now());
                    room.game_state.current_turn = Some(player_id);
                    state.broadcast_room_state(&room).await?;
                    
                    // Start a 3-second countdown
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(3)).await;
                        if let Ok(mut room) = state.get_room(&room_id).await {
                            room.game_state.status = GameStatus::InProgress;
                            room.game_state.turn_start_time = Some(Utc::now());
                            state.broadcast_room_state(&room).await.ok();
                        }
                    });
                }
            }
            GameMessage::MakeMove { room_id, player_id, number, position } => {
                let mut room = state.get_room(&room_id).await?;
                if room.game_state.status == GameStatus::InProgress 
                    && room.game_state.current_turn.as_ref() == Some(&player_id) {
                    
                    let move_result = Move {
                        player_id: player_id.clone(),
                        number,
                        position,
                        timestamp: Utc::now(),
                    };
                    
                    // Validate move and update game state
                    if let Some(player) = room.players.get_mut(&player_id) {
                        player.sequence.push(number);
                        player.last_move_time = Some(Utc::now());
                        
                        // Calculate score based on correctness and time
                        if let Some(turn_start) = room.game_state.turn_start_time {
                            let elapsed = Utc::now() - turn_start;
                            let seconds_remaining = 60 - elapsed.num_seconds();
                            if seconds_remaining > 0 {
                                player.score += seconds_remaining as i32; // Time bonus
                            }
                        }
                        
                        // Move to next player
                        if let Some(next_player) = room.players.keys()
                            .find(|&p| p != &player_id && room.players[p].is_connected) {
                            room.game_state.current_turn = Some(next_player.clone());
                            room.game_state.turn_start_time = Some(Utc::now());
                        }
                    }
                    
                    state.broadcast_room_state(&room).await?;
                }
            }
            GameMessage::Leave { room_id, player_id } => {
                let mut room = state.get_room(&room_id).await?;
                if let Some(player) = room.players.get_mut(&player_id) {
                    player.is_connected = false;
                }
                state.broadcast_room_state(&room).await?;
            }
            _ => {}
        }
        Ok(())
    }
}

pub async fn game_ws(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<GameState>,
) -> Result<HttpResponse, Error> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, stream)?;
    
    let (tx, mut rx) = mpsc::channel(32);
    let mut game_session = GameSession::new(tx);
    
    // Start heartbeat task
    let heartbeat_tx = tx.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
        loop {
            interval.tick().await;
            if heartbeat_tx.send(Message::Ping(Vec::new())).await.is_err() {
                break;
            }
        }
    });
    
    // Start main session loop
    tokio::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            if game_session.handle_message(msg, &state).await.is_err() {
                break;
            }
            
            if Instant::now().duration_since(game_session.heartbeat) > CLIENT_TIMEOUT {
                break;
            }
        }
        
        // Handle disconnection
        if let (Some(room_id), Some(player_id)) = (game_session.room_id, game_session.player_id) {
            if let Ok(mut room) = state.get_room(&room_id).await {
                if let Some(player) = room.players.get_mut(&player_id) {
                    player.is_connected = false;
                }
                state.broadcast_room_state(&room).await.ok();
            }
        }
    });
    
    Ok(response)
} 