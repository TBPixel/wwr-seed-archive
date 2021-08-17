use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub struct User {
    pub id: u64,
    pub discord: String,
    pub twitch: Option<String>,
    pub alias: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub seeds: Vec<Seed>,
    pub comments: Vec<Comment>,
}

pub struct Comment {
    pub id: u64,
    pub seed_id: u64,
    pub author: User,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Seed {
    pub id: u64,
    pub value: String,
    pub settings: SeedSettings,
    pub name: String,
    pub description: String,
    pub author: User,
    pub tags: Vec<String>,
    pub comments: Vec<Comment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct Preset {
    pub id: u64,
    pub name: String,
    pub settings: Settings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum Setting {
    Flag(bool),
    Select(String),
}

pub struct Settings(HashMap<String, Setting>);

pub enum SeedSettings {
    Preset(Preset),
    Settings(Settings),
}
