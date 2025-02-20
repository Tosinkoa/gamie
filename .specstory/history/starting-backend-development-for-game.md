## SpecStory

## Starting Backend Development for Game (20/02/2025, 18:08:54)

_**User**_

@gamie-backend 

Pls let start



---

_**Assistant**_

I'll help you get started with building the backend for the multiplayer number arrangement game. Let me first examine the current state of the backend project.

Let me check the contents of the Cargo.toml file to see what dependencies are already set up.

---

_****_

Read file: gamie-backend/Cargo.toml

---

_****_



Now let me check the current source code structure:

---

_****_

<details>
            <summary>Listed directory gamie-backend/src • **7** results</summary>
        
| Name |
|-------|
| 📄 `main.rs` |
| 📄 `error.rs` |
| 📄 `models` |
| 📄 `user` |
| 📄 `auth` |
| 📄 `config.rs` |
| 📄 `db.rs` |

</details>

---

_****_



Based on the requirements and current project structure, we need to implement several key components for the game backend. I'll help you set up the core game functionality. Let's start by creating the necessary modules and structures for room management and game state.

First, let's create a new module for game-related functionality:

---

_****_

```diff
- 
+ pub mod room;
+ pub mod state;
+ pub mod types;
+ 
+ use crate::error::AppError;
+ use lazy_static::lazy_static;
+ use serde::{Deserialize, Serialize};
+ use std::collections::HashMap;
+ use tokio::sync::RwLock;
+ 
+ lazy_static! {
+     static ref GAME_ROOMS: RwLock<HashMap<String, Room>> = RwLock::new(HashMap::new());
+ }
+ 
+ #[derive(Debug, Serialize, Deserialize, Clone)]
+ pub struct Room {
+     pub id: String,
+     pub name: String,
+     pub players: Vec<Player>,
+     pub game_state: Option<GameState>,
+     pub settings: RoomSettings,
+     pub created_by: String,
+     pub is_active: bool,
+ }
+ 
+ #[derive(Debug, Serialize, Deserialize, Clone)]
+ pub struct Player {
+     pub id: String,
+     pub username: String,
+     pub score: i32,
+     pub sequence: Vec<i32>,
+     pub is_connected: bool,
+     pub last_move_timestamp: i64,
+ }
+ 
+ #[derive(Debug, Serialize, Deserialize, Clone)]
+ pub struct GameState {
+     pub current_turn: String, // Player ID
+     pub correct_sequence: Vec<i32>,
+     pub turn_start_time: i64,
+     pub game_start_time: i64,
+     pub is_completed: bool,
+     pub winner: Option<String>, // Player ID
+ }
+ 
+ #[derive(Debug, Serialize, Deserialize, Clone)]
+ pub struct RoomSettings {
+     pub min_players: u32,
+     pub max_players: u32,
+     pub turn_duration_seconds: u32,
+     pub game_duration_minutes: u32,
+ }
+ 
+ impl Room {
+     pub fn new(id: String, name: String, created_by: String) -> Self {
+         Room {
+             id,
+             name,
+             players: Vec::new(),
+             game_state: None,
+             settings: RoomSettings {
+                 min_players: 2,
+                 max_players: 4,
+                 turn_duration_seconds: 60,
+                 game_duration_minutes: 15,
+             },
+             created_by,
+             is_active: true,
+         }
+     }
+ }
+ 
```

---

_****_



Now, let's create the room management module with the core functionality:

---

_****_

```diff
- 
+ use super::{GameState, Player, Room, GAME_ROOMS};
+ use crate::error::AppError;
+ use chrono::Utc;
+ use rand::seq::SliceRandom;
+ use rand::thread_rng;
+ 
+ pub async fn create_room(name: String, created_by: String) -> Result<Room, AppError> {
+     let mut rooms = GAME_ROOMS.write().await;
+     
+     // Check if user already has an active room
+     if rooms.values().any(|room| room.created_by == created_by && room.is_active) {
+         return Err(AppError::BadRequest("User already has an active room".into()));
+     }
+ 
+     let room_id = uuid::Uuid::new_v4().to_string();
+     let room = Room::new(room_id.clone(), name, created_by);
+     rooms.insert(room_id.clone(), room.clone());
+     
+     Ok(room)
+ }
+ 
+ pub async fn join_room(room_id: &str, player: Player) -> Result<Room, AppError> {
+     let mut rooms = GAME_ROOMS.write().await;
+     
+     let room = rooms.get_mut(room_id)
+         .ok_or_else(|| AppError::NotFound("Room not found".into()))?;
+ 
+     if !room.is_active {
+         return Err(AppError::BadRequest("Room is no longer active".into()));
+     }
+ 
+     if room.game_state.is_some() {
+         // Check if player was previously in the game
+         if !room.players.iter().any(|p| p.id == player.id) {
+             return Err(AppError::BadRequest("Cannot join an ongoing game".into()));
+         }
+     }
+ 
+     // Update player connection status if rejoining
+     if let Some(existing_player) = room.players.iter_mut().find(|p| p.id == player.id) {
+         existing_player.is_connected = true;
+         return Ok(room.clone());
+     }
+ 
+     if room.players.len() >= room.settings.max_players as usize {
+         return Err(AppError::BadRequest("Room is full".into()));
+     }
+ 
+     room.players.push(player);
+     Ok(room.clone())
+ }
+ 
+ pub async fn leave_room(room_id: &str, player_id: &str) -> Result<(), AppError> {
+     let mut rooms = GAME_ROOMS.write().await;
+     
+     let room = rooms.get_mut(room_id)
+         .ok_or_else(|| AppError::NotFound("Room not found".into()))?;
+ 
+     if let Some(game_state) = &room.game_state {
+         // If game is ongoing, just mark player as disconnected
+         if let Some(player) = room.players.iter_mut().find(|p| p.id == player_id) {
+             player.is_connected = false;
+         }
+ 
+         // Check if all players disconnected
+         if !room.players.iter().any(|p| p.is_connected) {
+             room.is_active = false;
+         }
+     } else {
+         // If game hasn't started, remove player completely
+         room.players.retain(|p| p.id != player_id);
+         
+         // Remove room if empty
+         if room.players.is_empty() {
+             rooms.remove(room_id);
+         }
+     }
+ 
+     Ok(())
+ }
+ 
+ pub async fn start_game(room_id: &str) -> Result<Room, AppError> {
+     let mut rooms = GAME_ROOMS.write().await;
+     
+     let room = rooms.get_mut(room_id)
+         .ok_or_else(|| AppError::NotFound("Room not found".into()))?;
+ 
+     if room.players.len() < room.settings.min_players as usize {
+         return Err(AppError::BadRequest("Not enough players to start".into()));
+     }
+ 
+     if room.game_state.is_some() {
+         return Err(AppError::BadRequest("Game already in progress".into()));
+     }
+ 
+     // Generate correct sequence
+     let mut sequence: Vec<i32> = (1..=9).collect();
+     let mut rng = thread_rng();
+     sequence.shuffle(&mut rng);
+ 
+     let now = Utc::now().timestamp();
+     room.game_state = Some(GameState {
+         current_turn: room.players[0].id.clone(),
+         correct_sequence: sequence,
+         turn_start_time: now,
+         game_start_time: now,
+         is_completed: false,
+         winner: None,
+     });
+ 
+     Ok(room.clone())
+ }
+ 
+ pub async fn get_room(room_id: &str) -> Result<Room, AppError> {
+     let rooms = GAME_ROOMS.read().await;
+     
+     rooms.get(room_id)
+         .cloned()
+         .ok_or_else(|| AppError::NotFound("Room not found".into()))
+ } 
```

---

_****_



Now let's create the game state management module:

---

_****_



---

_****_



---



---

