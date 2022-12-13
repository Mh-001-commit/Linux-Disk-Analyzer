/*#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
*/

#![allow(unused)]
use chrono::{NaiveDate, NaiveDateTime};
use clap::{App, Arg};
use indextree::{Arena, NodeId};
use serde::{Deserialize, Serialize};
use serde_indextree::Node;
use serde_json::{to_string, to_string_pretty};
use std::fs::metadata;
use std::{
    fs::{self},
    io::{Error, ErrorKind},
    path::Path,
};
//
use glob::glob;
use std::fs::symlink_metadata;
use std::os::unix::fs::MetadataExt;
use walkdir::{DirEntry, WalkDir};
//

#[derive(Serialize, Deserialize, Clone, Debug)]
struct PathNode {
    name: String,
    value: i128,
    relative_path: String,
    absolute_path: String,
    UserID: u32,
    actime: NaiveDateTime,
    modtime: NaiveDateTime,
    node_type: NodeType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum NodeType {
    File,
    Directory,
}
// fn start(invoke_message: String) -> Result<(), Error> {
//     if let directory_path = invoke_message {
//         let root_path = Path::new(&directory_path);

//         if root_path.is_dir() {
//             let mut absolute_path = std::env::current_dir()?;
//             absolute_path.push(root_path);

//             let arena = &mut Arena::new();

//             let root_node = arena.new_node(PathNode {
//                 name: directory_path.to_string(),
//                 relative_path: directory_path.to_string(),
//                 absolute_path: absolute_path.display().to_string(),
//                 UserID: (root_path.metadata().unwrap().uid()),
//                 value: i128::from(root_path.metadata().unwrap().size()),
//                 actime: NaiveDateTime::from_timestamp_opt(root_path.metadata().unwrap().atime(), 0)
//                     .unwrap(),
//                 modtime: NaiveDateTime::from_timestamp_opt(
//                     root_path.metadata().unwrap().mtime(),
//                     0,
//                 )
//                 .unwrap(),
//                 node_type: NodeType::Directory,
//             });

//             traverse(&directory_path, arena, root_node)?;
//             println!(
//                 "{}",
//                 to_string_pretty(&Node::new(root_node, arena)).unwrap()
//             );
//             Ok(())
//         } else {
//             eprintln!("Invalid directory.");
//             Result::Err(Error::new(
//                 ErrorKind::InvalidInput,
//                 format!("Invalid directory: {}", directory_path),
//             ))
//         }
//     } else {
//         Result::Err(Error::new(
//             ErrorKind::InvalidInput,
//             "No path provided.".to_string(),
//         ))
//     }
// }

// #[tauri::command]
// fn my_custom_command(invoke_message: String){

// }
#[tauri::command]
fn my_custom_command(invokeMessage: String) -> String {
    let directory_path = invokeMessage;
    let root_path = Path::new(&directory_path);

    let mut absolute_path = (std::env::current_dir()).unwrap();
    absolute_path.push(root_path);

    let arena = &mut Arena::new();

    let root_node = arena.new_node(PathNode {
        name: directory_path.to_string(),
        relative_path: directory_path.to_string(),
        absolute_path: absolute_path.display().to_string(),
        UserID: (root_path.metadata().unwrap().uid()),
        value: i128::from(root_path.metadata().unwrap().size()),
        actime: NaiveDateTime::from_timestamp_opt(root_path.metadata().unwrap().atime(), 0)
            .unwrap(),
        modtime: NaiveDateTime::from_timestamp_opt(root_path.metadata().unwrap().mtime(), 0)
            .unwrap(),
        node_type: NodeType::Directory,
    });

    traverse(&directory_path, arena, root_node);
    // let deserialze = (&Node::new(root_node, arena));
    // return deserialze;
    let deserialze = serde_json::to_string_pretty(&Node::new(root_node, arena));
    return deserialze.unwrap();
    // println!(
    //     "{}",
    //     to_string_pretty(&Node::new(root_node, arena)).unwrap())
}

fn main() {
    tauri::Builder::default()
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![my_custom_command, delete])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

fn traverse(path: &str, arena: &mut Arena<PathNode>, parent: NodeId) -> Result<(), Error> {
    let dir_listing = get_directory_listing(path);
    for entry in dir_listing {
        let temp_path = Path::new(entry.as_str());
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(temp_path);

        if temp_path.is_dir() {
            let dir_object = arena.new_node(PathNode {
                name: String::from(temp_path.file_name().unwrap().to_str().unwrap()),
                relative_path: String::from(entry.as_str()),
                absolute_path: absolute_path.display().to_string(),
                UserID: (temp_path.metadata().unwrap().uid()),
                value: i128::from(temp_path.metadata().unwrap().size()),
                actime: NaiveDateTime::from_timestamp_opt(temp_path.metadata().unwrap().atime(), 0)
                    .unwrap(),
                modtime: NaiveDateTime::from_timestamp_opt(
                    temp_path.metadata().unwrap().mtime(),
                    0,
                )
                .unwrap(),
                node_type: NodeType::Directory,
            });

            parent.append(dir_object, arena);
            traverse(entry.as_str(), arena, dir_object)?;
        } else {
            let file_object = arena.new_node(PathNode {
                name: String::from(temp_path.file_name().unwrap().to_str().unwrap()),
                relative_path: String::from(entry.as_str()),
                absolute_path: absolute_path.display().to_string(),
                UserID: (temp_path.metadata().unwrap().uid()),
                value: i128::from(temp_path.metadata().unwrap().size()),
                actime: NaiveDateTime::from_timestamp_opt(temp_path.metadata().unwrap().atime(), 0)
                    .unwrap(),
                modtime: NaiveDateTime::from_timestamp_opt(
                    temp_path.metadata().unwrap().mtime(),
                    0,
                )
                .unwrap(),
                node_type: NodeType::File,
            });

            parent.append(file_object, arena);
        }
    }

    Ok(())
}

#[tauri::command]
fn delete(path: String) {
    let temp = path.clone();
    let md = metadata(temp).unwrap();

    if (md.is_file()) {
        fs::remove_file(path).expect("File delete failed");
        println!("File deleted successfully!");
    } else {
        fs::remove_dir_all(path).expect("Error removing directory");
        println!("Directory removed successfully!");
    }
}

fn get_directory_listing(directory_path: &str) -> Vec<String> {
    fs::read_dir(directory_path)
        .unwrap()
        .map(|x| x.unwrap().path().to_str().unwrap().to_string())
        .collect()
}
