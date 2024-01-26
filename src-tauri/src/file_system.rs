use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;

const TMP_DIR: &str = "/Users/lokep/Desktop/project/github/leetcode/";
const TMP_FILE: &str = "/Users/lokep/Desktop/project/github/leetcode/readme.md";

#[tauri::command]
pub fn is_file_exist(path: &str) -> bool {
    Path::new(path).exists()
}

#[tauri::command]
pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

#[tauri::command]
pub fn read_file(path: &str) -> String {
    let mut contents = String::new();

    if !is_file_exist(path) {
        return contents;
    }

    let mut file = std::fs::File::open(path).unwrap();
    file.read_to_string(&mut contents).unwrap();

    contents
}

#[tauri::command]
pub fn read_dir(path: &str) -> Vec<String> {
    let mut contents = Vec::new();

    if !is_dir(path) {
        return contents;
    }

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        contents.push(entry.file_name().to_str().unwrap().to_string());
    }
    contents
}

#[tauri::command]
pub fn create_dir(path: &str) {
    if is_dir(path) {
        return;
    }

    std::fs::create_dir_all(path).unwrap();
}

// TODO 希望可以连带创建多级目录
#[tauri::command]
pub fn create_file(path: &str) {
    // let np = Path::new(path);

    if is_file_exist(path) {
        return;
    }

    std::fs::File::create(path).unwrap();
}

#[tauri::command]
pub fn write_file(path: &str, content: &str) {
    if !is_file_exist(path) {
        create_file(path);
    }

    std::fs::write(path, content).unwrap();
}

#[tauri::command]
pub fn update_file(path: &str, content: &str) {
    if !is_file_exist(path) {
        create_file(path);
    }

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    f.write(content.as_bytes()).unwrap();
}

#[tauri::command]
pub fn delete_file(path: &str) {
    std::fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn del_dir(path: &str) {
    std::fs::remove_dir_all(path).unwrap();
}

// fn main() {
// // 测试文件读取
// let file_content = read_file(TMP_FILE);
// println!("file_content = {}", file_content);

// // 测试目录读取
// let dir_content = read_dir(TMP_DIR);
// println!("dir_content = {:#?}", dir_content);

// // 测试目录创建
// create_dir("./test/");

// // 测试文件创建
// create_file("./test/test.md");

// // 测试文件写入
// write_file("./test/test.md", "hello");

// // 测试文件更新
// update_file("./test/test.md", "world");

// // 测试文件删除
// delete_file("./test/test.md");

// 测试目录删除
// del_dir("./test/");
// }
