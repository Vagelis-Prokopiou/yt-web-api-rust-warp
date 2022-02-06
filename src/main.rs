use tokio::runtime::Builder;
use warp::Filter;

use api_warp::and_then_get_users;
use api_warp::then_get_users;

fn main() {
    let logical_cpu_cores = num_cpus::get();
    let physical_cpu_cores = num_cpus::get_physical();

    let runtime = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(logical_cpu_cores)
        // .worker_threads(physical_cpu_cores)
        .build()
        .unwrap();

    // Execute the future, blocking the current thread until completion
    runtime.block_on(async {
        let users_route = warp::path("users")
            .and(warp::path::end())
            // .and_then(and_then_get_users);
            .then(then_get_users); // Best result

        println!("\nServer running at: http://127.0.0.1:8082/users");
        warp::serve(users_route).run(([127, 0, 0, 1], 8082)).await;
    });
}
