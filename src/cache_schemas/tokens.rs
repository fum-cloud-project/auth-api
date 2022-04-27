#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
    REFRESH,
    ACCESS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub user_access_level: i32,
    pub token_use_case: TokenType,
    pub exp: i64,
}

impl Claims {
    fn new(
        user_id: String,
        user_access_level: i32,
        token_use_case: TokenType,
        dur: chrono::Duration,
    ) -> Claims {
        Claims {
            user_id,
            user_access_level,
            token_use_case,
            exp: chrono::Utc::now()
                .checked_add_signed(dur)
                .unwrap()
                .timestamp(),
        }
    }
}