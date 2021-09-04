use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coin {
    pub id: String,
    pub current_price: f32,
    pub ath: f32
}
