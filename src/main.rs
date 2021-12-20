use warp::Filter;

use api::get_users;

#[tokio::main]
async fn main() {
    let users = warp::path!("users").map(|| warp::reply::json(&get_users()));
    warp::serve(users).run(([127, 0, 0, 1], 8084)).await;
}
