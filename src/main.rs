mod storage;
mod address;
mod names;
mod update;
mod mybincode;
mod jwt;
mod handlers;

use warp::{Filter, path, path::param};

macro_rules! route {
    ($x:ty) => (impl Filter<Extract = ($x,), Error = warp::Rejection> + Clone)
}

fn login() -> route!(impl warp::Reply) {
    path("login").and(param())
        .and_then(handlers::login)
}

fn auth() -> route!(std::path::PathBuf) {
    warp::cookie("token")
        .and_then(handlers::auth)
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    warp::serve(login().or(api()).or(warp::fs::dir("html")))
        .run(([127, 0, 0, 1], 3030))
        .await;
    Ok(())
}

fn api() -> route!(impl warp::Reply) {
    let get_api = warp::get()
        .and(path("top").and(auth())
            .then(handlers::list_top_nodes)
        .or(path("node").and(auth()).and(param())
            .then(handlers::list_node_inputs))
        .or(path("names").and(auth()).and(param())
            .then(handlers::list_node_names)));
    let post_api = warp::post()
        .and(path("create").and(auth()).and(param())
            .map(handlers::create_node));
    let put_api = warp::put()
        .and(path("name").and(auth()).and(param()).and(param())
            .map(handlers::update_node_names)
        .or(path("import").and(auth()).and(param())
            .map(handlers::import_all))
        .or(path("export").and(auth()).and(param()).and(param())
            .map(handlers::export_node)));
    let delete_api = warp::delete()
        .and(path("clear").and(auth())
            .map(handlers::clear_project));
    get_api.or(post_api).or(put_api).or(delete_api)
}
