use crate::actix::Addr;
use crate::actors::cache::tokens::{
    AddNewPair, DelToken, DelTokenPair, IsTokenOwnerRevoked, TokenExists, TokenPairExists,
};
use crate::actors::cache::CacheActor;
use crate::cache_schemas::tokens::{Claims, TokenType};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::sync::Arc;

pub async fn gen_tokens(
    user_id: String,
    user_access_level: i32,
    cache: Addr<CacheActor>,
    secret: Arc<&[u8]>,
) -> Result<(String, String), ()> {
    let refresh_token = Claims::new(
        user_id.clone(),
        user_access_level,
        TokenType::REFRESH,
        chrono::Duration::days(1),
    );
    let access_token = Claims::new(
        user_id,
        user_access_level,
        TokenType::ACCESS,
        chrono::Duration::seconds(10),
    );

    let refresh_token_string = match encode(
        &Header::default(),
        &refresh_token,
        &EncodingKey::from_secret(*secret),
    ) {
        Ok(val) => val,
        _ => {
            return Err(());
        }
    };

    let access_token_string = match encode(
        &Header::default(),
        &access_token,
        &EncodingKey::from_secret(*secret),
    ) {
        Ok(val) => val,
        _ => {
            return Err(());
        }
    };

    match cache
        .send(AddNewPair {
            access_tok: access_token_string.clone(),
            refresh_tok: refresh_token_string.clone(),
            access_exp: 10,
            refresh_exp: 24 * 60 * 60,
        })
        .await
    {
        Ok(Ok(_)) => {
            return Ok((access_token_string, refresh_token_string));
        }
        _ => {
            return Err(());
        }
    }
}

#[inline(always)]
fn is_token_expired(exp: i64, length: i64) -> bool {
    chrono::Utc::now()
        .naive_utc()
        .signed_duration_since(chrono::NaiveDateTime::from_timestamp(exp, 0))
        .num_seconds()
        >= length
}

pub async fn verify_token(
    token: String,
    secret: Arc<&[u8]>,
    cache: Addr<CacheActor>,
) -> Result<(String, i32), ()> {
    let token_s = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(*secret),
        &Validation::default(),
    ) {
        Ok(val) => val,
        _ => {
            return Err(());
        }
    };
    if token_s.claims.token_use_case != TokenType::ACCESS
        || is_token_expired(token_s.claims.exp, 10)
    {
        return Err(());
    }

    match cache
        .clone()
        .send(IsTokenOwnerRevoked {
            id: token_s.claims.user_id.clone(),
        })
        .await
    {
        Ok(Ok(s)) if s => {
            return Err(());
        }
        _ => {}
    }

    match cache
        .clone()
        .send(TokenExists {
            token: token.clone(),
        })
        .await
    {
        Ok(Ok(s)) if s => {}
        _ => {
            return Err(());
        }
    }

    match cache
        .clone()
        .send(DelToken {
            token: token.clone(),
        })
        .await
    {
        Ok(Ok(_)) => {
            return Ok((token_s.claims.user_id, token_s.claims.user_access_level));
        }
        _ => {
            return Err(());
        }
    }
}

pub async fn refresh_token(
    token: String,
    secret: Arc<&[u8]>,
    cache: Addr<CacheActor>,
) -> Result<(String, String), u8> {
    let token_s = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(*secret),
        &Validation::default(),
    ) {
        Ok(val) => val,
        _ => {
            return Err(0);
        }
    };

    if token_s.claims.token_use_case != TokenType::REFRESH
        || is_token_expired(token_s.claims.exp, 24 * 60 * 60)
    {
        return Err(0);
    }

    match cache
        .clone()
        .send(TokenExists {
            token: token.clone(),
        })
        .await
    {
        Ok(Ok(s)) if s => {}
        Ok(Ok(s)) if !s => {
            return Err(0);
        }
        _ => {
            return Err(1);
        }
    }

    match cache
        .clone()
        .send(TokenPairExists {
            token: token.clone(),
        })
        .await
    {
        Ok(Ok(s)) if s == false => {}
        Ok(Ok(s)) if s == true => {
            //Access token has not expired yet
            return Err(2);
        }
        _ => {
            return Err(1);
        }
    }

    match cache.clone().send(DelToken { token: token }).await {
        Ok(_) => {}
        _ => {
            return Err(1);
        }
    }
    match gen_tokens(
        token_s.claims.user_id,
        token_s.claims.user_access_level,
        cache.clone(),
        secret,
    )
    .await
    {
        Err(_) => Err(0),
        Ok(s) => Ok(s),
    }
}

pub async fn revoke_token(token: String, cache: Addr<CacheActor>) -> Result<bool, ()> {
    match cache.clone().send(DelTokenPair { token: token }).await {
        Ok(Ok(_)) => Ok(true),
        Ok(Err(_)) => Ok(false),
        _ => {
            return Err(());
        }
    }
}

#[allow(dead_code)]
pub async fn verify_token_and_get_user_id(
    token: String,
    secret: Arc<&[u8]>,
) -> Result<bson::oid::ObjectId, ()> {
    let token_s = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(*secret),
        &Validation::default(),
    ) {
        Ok(val) => val,
        _ => {
            return Err(());
        }
    };
    if token_s.claims.token_use_case != TokenType::ACCESS
        || is_token_expired(token_s.claims.exp, 10)
    {
        return Err(());
    }

    Ok(bson::oid::ObjectId::parse_str(token_s.claims.user_id).unwrap())
}
