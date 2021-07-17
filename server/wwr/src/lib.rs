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
}

pub struct Seed {
    pub value: String,
    pub author: User,
    pub tags: Vec<String>,
    pub preset: Option<Preset>,
    pub settings: Option<Settings>,
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
