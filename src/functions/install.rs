use crate::functions::pull;
use crate::internal::repos::*;
use std::process::Command;
use crate::internal::returncode_eval::*;

pub fn install(name: &str) {
    pull::pull(name);
    let mut podman = Command::new("podman");
    let mut distrobox = Command::new("distrobox");
    exec_eval(
        podman.arg("load").arg("-i").arg(format!("{}/.cache/distromgr/{}-{}",env!("HOME"), get_repo_info()[0], search_repo(name)[0])).status(),
        format!("Loading {} from {}", name, get_repo_info()[3]).as_str()
    );
    exec_eval(
        distrobox.arg("create").arg("--name").arg(name).arg("--image").arg(format!("localhost/{}:latest", name)).status(),
        format!("Installing {} from {} as distrobox container", name, get_repo_info()[3]).as_str()
    )
}