use tokio::runtime::Builder;
use warp::Filter;

fn main() {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(8)
        .build()
        .unwrap();

    // Execute the future, blocking the current thread until completion
    runtime.block_on(async {
        let users_route = warp::path("api")
            .and(warp::path("v1"))
            .and(warp::path("users"))
            .and(warp::path::end())
            .map(|| warp::reply::json(&api_warp::get_users()));

        warp::serve(users_route).run(([127, 0, 0, 1], 8082)).await;
    });
}
