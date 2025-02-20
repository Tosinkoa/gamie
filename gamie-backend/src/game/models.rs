use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub creator_id: String,
    pub players: HashMap<String, Player>,
    pub game_state: GameState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub score: i32,
    pub sequence: Vec<i32>,
    pub is_connected: bool,
    pub last_move_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameStatus {
    Waiting,
    Starting,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub status: GameStatus,
    pub current_turn: Option<String>,
    pub correct_sequence: Vec<i32>,
    pub available_numbers: Vec<i32>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub turn_start_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub player_id: String,
    pub number: i32,
    pub position: usize,
    pub timestamp: DateTime<Utc>,
}

impl Room {
    pub fn new(id: String, name: String, creator_id: String) -> Self {
        Room {
            id,
            name,
            creator_id,
            players: HashMap::new(),
            game_state: GameState {
                status: GameStatus::Waiting,
                current_turn: None,
                correct_sequence: Vec::new(),
                available_numbers: Vec::new(),
                start_time: None,
                end_time: None,
                turn_start_time: None,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn add_player(&mut self, id: String, username: String) -> bool {
        if self.game_state.status != GameStatus::Waiting {
            return false;
        }

        self.players.insert(
            id.clone(),
            Player {
                id,
                username,
                score: 0,
                sequence: Vec::new(),
                is_connected: true,
                last_move_time: None,
            },
        );
        self.updated_at = Utc::now();
        true
    }

    pub fn remove_player(&mut self, player_id: &str) {
        self.players.remove(player_id);
        self.updated_at = Utc::now();
    }

    pub fn can_start(&self) -> bool {
        self.players.len() >= 2 && self.game_state.status == GameStatus::Waiting
    }
}

impl GameState {
    pub fn generate_sequence(&mut self, length: usize) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        let mut numbers: Vec<i32> = (1..=100).collect();
        numbers.shuffle(&mut rng);
        self.correct_sequence = numbers.iter().take(length).cloned().collect();

        // Generate available numbers (2 options per position)
        let mut available = Vec::new();
        for &correct in &self.correct_sequence {
            let mut options = vec![correct];
            while options.len() < 2 {
                let random = (1..=100).collect::<Vec<i32>>().choose(&mut rng).unwrap();
                if !options.contains(random) {
                    options.push(*random);
                }
            }
            options.shuffle(&mut rng);
            available.extend(options);
        }
        self.available_numbers = available;
    }
}
