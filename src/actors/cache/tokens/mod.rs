use crate::actix::ResponseFuture;
use crate::actix::{Handler, Message};
use crate::actors::cache::CacheActor;
#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct AddNewPair {
    pub access_tok: String,
    pub refresh_tok: String,
    pub access_exp: i64,
    pub refresh_exp: i64,
}

#[derive(Message)]
#[rtype(result = "Result<bool, ()>")]
pub struct TokenExists {
    pub token: String,
}

#[derive(Message)]
#[rtype(result = "Result<bool, ()>")]
pub struct TokenPairExists {
    pub token: String,
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct DelToken {
    pub token: String,
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct DelTokenPair {
    pub token: String,
}

#[derive(Message)]
#[rtype(result = "Result<bool, ()>")]
pub struct IsTokenOwnerRevoked {
    pub id: String,
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct BanUser {
    pub id: String,
    pub dur: i64,
}

impl Handler<IsTokenOwnerRevoked> for CacheActor {
    type Result = ResponseFuture<Result<bool, ()>>;

    fn handle(&mut self, msg: IsTokenOwnerRevoked, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            match redis::cmd("GET")
                .arg(&msg.id)
                .query_async(&mut connection)
                .await
            {
                Ok::<Option<String>, redis::RedisError>(Some(_)) => Ok(true),
                Ok::<Option<String>, redis::RedisError>(None) => Ok(false),
                _ => Err(()),
            }
        })
    }
}

impl Handler<BanUser> for CacheActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: BanUser, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            match redis::cmd("SET")
                .arg(&[&msg.id, &msg.id, "EX", &msg.dur.to_string()])
                .query_async(&mut connection)
                .await
            {
                Ok::<String, redis::RedisError>(_) => Ok(()),
                Err(_) => Err(()),
            }
        })
    }
}

impl Handler<AddNewPair> for CacheActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: AddNewPair, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            match redis::pipe()
                .atomic()
                .cmd("SET")
                .arg(&[
                    &msg.access_tok,
                    &msg.refresh_tok,
                    "EX",
                    &msg.access_exp.to_string(),
                ])
                .cmd("SET")
                .ignore()
                .arg(&[
                    &msg.refresh_tok,
                    &msg.access_tok,
                    "EX",
                    &msg.refresh_exp.to_string(),
                ])
                .ignore()
                .query_async(&mut connection)
                .await
            {
                Ok::<Vec<String>, redis::RedisError>(_) => Ok(()),
                Err(_) => Err(()),
            }
        })
    }
}

impl Handler<TokenExists> for CacheActor {
    type Result = ResponseFuture<Result<bool, ()>>;

    fn handle(&mut self, msg: TokenExists, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            match redis::cmd("GET")
                .arg(&msg.token)
                .query_async(&mut connection)
                .await
            {
                Ok::<Option<String>, redis::RedisError>(Some(_)) => Ok(true),
                Ok::<Option<String>, redis::RedisError>(None) => Ok(false),
                _ => Err(()),
            }
        })
    }
}

impl Handler<TokenPairExists> for CacheActor {
    type Result = ResponseFuture<Result<bool, ()>>;

    fn handle(&mut self, msg: TokenPairExists, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            let pair = match redis::cmd("GET")
                .arg(&msg.token)
                .query_async(&mut connection)
                .await
            {
                Ok::<Option<String>, redis::RedisError>(Some(s)) => s,
                Ok::<Option<String>, redis::RedisError>(None) => {
                    return Ok(false);
                }
                _ => {
                    return Err(());
                }
            };
            match redis::cmd("GET")
                .arg(&pair)
                .query_async(&mut connection)
                .await
            {
                Ok::<Option<String>, redis::RedisError>(Some(_)) => Ok(true),
                Ok::<Option<String>, redis::RedisError>(None) => Ok(false),
                _ => Err(()),
            }
        })
    }
}

impl Handler<DelToken> for CacheActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: DelToken, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            match redis::cmd("DEL")
                .arg(&msg.token)
                .query_async(&mut connection)
                .await
            {
                Ok::<Option<i32>, redis::RedisError>(Some(i)) if i == 1 => Ok(()),
                _ => Err(()),
            }
        })
    }
}

impl Handler<DelTokenPair> for CacheActor {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, msg: DelTokenPair, _: &mut Self::Context) -> Self::Result {
        let mut connection = self.0.clone();
        Box::pin(async move {
            match redis::cmd("GET")
                .arg(&msg.token)
                .query_async(&mut connection.clone())
                .await
            {
                Ok::<Option<String>, redis::RedisError>(Some(tok)) => {
                    match redis::cmd("DEL")
                        .arg(&tok)
                        .query_async(&mut connection.clone())
                        .await
                    {
                        Ok::<Option<i32>, redis::RedisError>(Some(i)) if i == 1 => {
                            match redis::cmd("DEL")
                                .arg(&msg.token)
                                .query_async(&mut connection)
                                .await
                            {
                                Ok::<Option<i32>, redis::RedisError>(Some(i)) if i == 1 => Ok(()),
                                _ => {
                                    return Err(());
                                }
                            }
                        }
                        _ => {
                            match redis::cmd("DEL")
                                .arg(&msg.token)
                                .query_async(&mut connection)
                                .await
                            {
                                Ok::<Option<i32>, redis::RedisError>(Some(i)) if i == 1 => Ok(()),
                                _ => {
                                    return Err(());
                                }
                            }
                        }
                    }
                }
                _ => Err(()),
            }
        })
    }
}
