use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Country {
    pub id: String,
    pub name: String,
    pub capital: String,
    pub timezone: String,
    pub population: String
}

