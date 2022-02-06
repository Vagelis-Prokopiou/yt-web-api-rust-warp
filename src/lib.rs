use std::convert::Infallible;

use warp::reply::Json;
use warp::{Rejection, Reply};

use crate::models::User;

pub mod models;

pub async fn and_then_get_users() -> Result<impl warp::Reply, Infallible> {
    let users = tokio::task::spawn_blocking(|| {
        let mut users = Vec::with_capacity(1000);
        for index in 1..1001_u16 {
            users.push(User {
                Id: index,
                Age: 25,
                First_Name: format!("First_Name{}", index),
                Last_Name: format!("Last_Name{}", index),
                Framework: "Rust (Warp)".to_owned(),
            })
        }
        users
    })
    .await
    .unwrap();

    Ok(warp::reply::json(&users))
}

pub async fn then_get_users() -> impl warp::Reply {
    let users = tokio::task::spawn_blocking(|| {
        let mut users = Vec::with_capacity(1000);
        for index in 1..1001_u16 {
            users.push(User {
                Id: index,
                Age: 25,
                First_Name: format!("First_Name{}", index),
                Last_Name: format!("Last_Name{}", index),
                Framework: "Rust (Warp)".to_owned(),
            })
        }
        users
    })
    .await
    .unwrap();

    warp::reply::json(&users)
}
