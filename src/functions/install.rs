use crate::functions::pull;
use std::process::Command;
use crate::internal::returncode_eval::*;

pub fn install(name: &str) {
    pull::pull(name);
    let mut podman = Command::new("podman");
    podman.arg("load").arg("-i").arg(format!("{}/.cache/distromgr/{}-{}",env!("HOME"), get_repo_info()[0], search_repo(name)[0]));
}