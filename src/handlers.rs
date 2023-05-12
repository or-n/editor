use crate::address::Address;
use crate::storage;
use crate::jwt;
use crate::names;
use crate::update;
use crate::address;

use warp::reply;
use std::path::{Path, PathBuf};

use tokio::fs::read;

type WebResult<T> = Result<T, warp::Rejection>;

#[derive(Debug)]
struct Unauthorized(jsonwebtoken::errors::Error);

impl warp::reject::Reject for Unauthorized {}

fn unauthorized(error: jsonwebtoken::errors::Error) -> warp::Rejection {
    warp::reject::custom(Unauthorized(error))
}

pub struct Addresses(Vec<Address>);

impl std::str::FromStr for Addresses {
    type Err = address::FromStrError;

    fn from_str(s: &str) -> Result<Addresses, Self::Err> {
        Ok(Addresses(s.split(',').map(Address::from_str).flatten().collect()))
    }
}

pub async fn login(username: String) -> WebResult<impl warp::Reply> {
    let token = jwt::token(username.clone()).map_err(unauthorized)?;
    Ok(reply::with_header(reply::json(&username), "Set-Cookie", jwt::cookie(token)))
}

pub async fn auth(token: String) -> WebResult<PathBuf> {
    let username = jwt::username(token).map_err(unauthorized)?;
    Ok(PathBuf::from(username))
}

pub async fn list_top_nodes(path: PathBuf) -> Vec<u8> {
    storage::ensure_project_structure(&path).unwrap();
    read(storage::top_nodes(&path)).await.unwrap()
}

pub async fn list_node_inputs(path: PathBuf, node: Address) -> Vec<u8> {
    read(storage::node_inputs(&path, &node)).await.unwrap()
}

pub async fn list_node_names(path: PathBuf, node: Address) -> Vec<u8> {
    read(storage::node_names(&path, &node)).await.unwrap()
}

pub fn create_node(path: PathBuf, inputs: Addresses) -> Vec<u8> {
    let result = storage::create_node(&path, &inputs.0).unwrap();
    bincode::serialize(&result).unwrap()
}

pub fn update_node_names(path: PathBuf, node: Address, name: String) -> Vec<u8> {
    let path = &storage::node_names(&path, &node);
    let update = update::string_empty_option(names::toggle(&name));
    storage::update::optional_file(path, update).unwrap();
    vec![]
}

pub fn import_all(path: PathBuf, base_name: String) -> Vec<u8> {
    let source = Path::new(&base_name);
    storage::ensure_project_structure(source).unwrap();
    let migrated = storage::migrate_all(source, &path).unwrap();
    migrated.to_le_bytes().to_vec()
}

pub fn export_node(path: PathBuf, base_name: String, node: Address) -> Vec<u8> {
    let target = Path::new(&base_name);
    storage::ensure_project_structure(target).unwrap();
    let migrated = storage::migrate(&path, target, &node).unwrap();
    migrated.to_le_bytes().to_vec()
} 

pub fn clear_project(path: PathBuf) -> Vec<u8> {
    storage::clear_project(&path).unwrap();
    vec![]
}
