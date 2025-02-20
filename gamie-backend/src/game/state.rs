use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;

use crate::game::models::{Room, GameMessage};
use crate::error::Error;

#[derive(Clone)]
pub struct GameState {
    rooms: Arc<RwLock<HashMap<String, Room>>>,
    connections: Arc<RwLock<HashMap<String, Vec<mpsc::Sender<Message>>>>>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_room(&self, room_id: String, name: String, creator_id: String) -> Result<Room, Error> {
        let room = Room::new(room_id.clone(), name, creator_id);
        self.rooms.write().await.insert(room_id, room.clone());
        Ok(room)
    }

    pub async fn get_room(&self, room_id: &str) -> Result<Room, Error> {
        self.rooms
            .read()
            .await
            .get(room_id)
            .cloned()
            .ok_or_else(|| Error::NotFound("Room not found".into()))
    }

    pub async fn update_room(&self, room: Room) -> Result<(), Error> {
        self.rooms.write().await.insert(room.id.clone(), room);
        Ok(())
    }

    pub async fn delete_room(&self, room_id: &str) -> Result<(), Error> {
        self.rooms.write().await.remove(room_id);
        Ok(())
    }

    pub async fn register_connection(&self, room_id: String, tx: mpsc::Sender<Message>) {
        let mut connections = self.connections.write().await;
        connections
            .entry(room_id)
            .or_insert_with(Vec::new)
            .push(tx);
    }

    pub async fn remove_connection(&self, room_id: &str, tx: &mpsc::Sender<Message>) {
        if let Some(connections) = self.connections.write().await.get_mut(room_id) {
            connections.retain(|conn| !conn.same_channel(tx));
        }
    }

    pub async fn broadcast_room_state(&self, room: &Room) -> Result<(), Error> {
        let message = serde_json::to_string(&GameMessage::GameState {
            room: room.clone(),
        })?;

        if let Some(connections) = self.connections.read().await.get(&room.id) {
            for tx in connections {
                tx.send(Message::Text(message.clone())).await.ok();
            }
        }

        // Clean up empty rooms
        if room.players.values().all(|p| !p.is_connected) {
            self.delete_room(&room.id).await?;
        }

        Ok(())
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
} 