use crate::internal::repos::*;
use std::process::Command;
use crate::internal::returncode_eval::*;

pub fn pull(name: &str) {
    let mut cmd = Command::new("wget");
    let repo = get_repo_info();
    let queries = search_repo(name);
    println!("here");
    exec_eval(
        cmd.arg("-q").arg("-O").arg(format!("{}/.cache/distromgr/{}-{}",env!("HOME"), repo[0], queries[0])).arg(format!("{}{}",repo[3], queries[0])).status(),
        format!("Downloading {} from {}", name, repo[3]).as_str()
    );
    println!("{:?}", get_repo_info());
    println!("{:?}", search_repo(name));
}