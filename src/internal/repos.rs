use toml::Value;
use std::fs::File;
use std::io::prelude::*;


pub fn parse_repo() {
    let mut repo_file = String::new();
    let mut file = File::open(format!("{}/.cache/distromgr/repo.toml", env!("HOME")).as_str()).unwrap();
    file.read_to_string(&mut repo_file).unwrap();
    let repo_parsed: Value = toml::from_str(&repo_file).unwrap();
    let repo_list = repo_parsed["package"].as_array().unwrap();
    for repo in repo_list {
        println!("Distropackage: {}", repo["name"].as_str().unwrap());
        println!("Description: {}", repo["distro"].as_str().unwrap());
        println!("Version: {}", repo["creation-date"].as_str().unwrap());
    }
}

pub fn get_repo_info() -> Vec<String> {
    let mut repo_file = String::new();
    let mut file = File::open(format!("{}/.cache/distromgr/repo.toml", env!("HOME")).as_str()).unwrap();
    file.read_to_string(&mut repo_file).unwrap();
    let repo_parsed: Value = toml::from_str(&repo_file).unwrap();
    return vec![
        repo_parsed["name"].as_str().unwrap().to_string(),
        repo_parsed["packagecount"].as_str().unwrap().to_string(),
        repo_parsed["maintainer"].as_str().unwrap().to_string(),
        repo_parsed["fetch-url"].as_str().unwrap().to_string(),
    ];
}

pub fn search_repo(query: &str) -> Vec<String> {
    let mut repo_file = String::new();
    let mut file = File::open(format!("{}/.cache/distromgr/repo.toml", env!("HOME")).as_str()).unwrap();
    file.read_to_string(&mut repo_file).unwrap();
    let repo_file: Value = toml::from_str(&repo_file).unwrap();
    let repo_list = repo_file["package"].as_array().unwrap();
    for repo in repo_list {
        if repo["name"].as_str().unwrap() == query {
            return vec![
                repo["name"].to_string(),
                repo["distro"].to_string(),
                repo["creation-date"].to_string(),
            ];
        }
    }
    return vec![];
}