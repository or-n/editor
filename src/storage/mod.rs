pub mod update;

use crate::mybincode::*;

use crate::address;
use address::Address;

use std::fs::{remove_dir_all, create_dir, write, read};
use std::path::{Path, PathBuf};
use std::io::Result as IO;

const TOP_NODES: &str = "top_nodes";
const NODES: &str = "nodes";
const NAMES: &str = "names";

pub fn ensure_project_structure(path: &Path) -> IO<()> {
    fn ensure<P: AsRef<Path>>(path: P, f: impl Fn(P) -> IO<()>) -> IO<()> {
        if !path.as_ref().exists() {
            f(path)?;
        }
        Ok(())
    }
    ensure(path, create_dir)?;
    ensure(path.join(NODES), create_dir)?;
    ensure(path.join(NAMES), create_dir)?;
    ensure(path.join(TOP_NODES), move |top_nodes_path| {
        write(top_nodes_path, [])?;
        let (_leaf, _creation_status) = create_node(path, &vec![])?;
        Ok(())
    })
}

pub fn clear_project(path: &Path) -> IO<()> {
    if path.exists() {
        remove_dir_all(path)?;
    }
    ensure_project_structure(path)
}

pub fn top_nodes(path: &Path) -> PathBuf {
    path.join(TOP_NODES)
}

pub fn node_inputs(path: &Path, node: &Address) -> PathBuf {
    path.join(NODES).join(node.to_string())
}

pub fn node_names(path: &Path, node: &Address) -> PathBuf {
    path.join(NAMES).join(node.to_string())
}

#[derive(Debug, serde::Serialize)]
pub enum CreationStatus {
    AlreadyExisted,
    Created
}

pub fn create_node(path: &Path, inputs: &Vec<Address>) -> IO<(Address, CreationStatus)> {
    let node = address::hash(inputs);
    let inputs_path = &node_inputs(path, &node);
    if inputs_path.exists() {
        Ok((node, CreationStatus::AlreadyExisted))
    } else {
        write(inputs_path, set_vec(inputs))?;
        let top_nodes_path = &top_nodes(path);
        let mut top_nodes = get_vec(&read(top_nodes_path)?).unwrap();
        top_nodes.retain(|top_node| !inputs.contains(top_node));
        top_nodes.push(node.clone());
        write(top_nodes_path, set_vec(&top_nodes))?;
        Ok((node, CreationStatus::Created))
    }
}

pub fn migrate(source: &Path, target: &Path, node: &Address) -> IO<u32> {
    if node_inputs(target, node).exists() {
        return Ok(0)
    }
    let inputs = get_vec(&read(&node_inputs(source, node))?).unwrap();
    let migrated = migrate_many(source, target, inputs.iter())?;
    let (_node, _creation_status) = create_node(target, &inputs)?;
    Ok(migrated + 1)
}

pub fn migrate_many<'a, I>(source: &Path, target: &Path, nodes: I) -> IO<u32>
where I: Iterator<Item = &'a Address> {
    Ok(nodes.flat_map(|node| migrate(source, target, node)).sum())
}

pub fn migrate_all(source: &Path, target: &Path) -> IO<u32> {
    let top_nodes = get_vec(&read(&top_nodes(source))?).unwrap();
    migrate_many(source, target, top_nodes.iter())
}
