mod functions;
mod internal;
//use internal::*;
use functions::*;
use clap::{App, Arg, SubCommand};
fn main() {
    let matches = App::new("pull")
        .version("0.1.0")
        .subcommand(
            SubCommand::with_name("pull")
            .about("Pulls a new container image from the remote")
            .arg(
                Arg::with_name("image")
                .help("The image to pull")
                .required(true)
                .index(1)
            )
        )
        .subcommand(
            SubCommand::with_name("backup")
            .about("backup a container and create an image")
            .arg(
                Arg::with_name("container")
                .help("The container to backup")
                .required(true)
                .index(1)
            )
        )
        .subcommand(
            SubCommand::with_name("install")
            .about("install a container from a repo")
            .arg(
                Arg::with_name("name")
                .help("The name of the container package")
                .required(true)
                .index(1)
            )
        )
        .subcommand(
            SubCommand::with_name("list")
            .about("list all installed containers")
        )
        .subcommand(
            SubCommand::with_name("search")
            .about("search for a container in the repo")
            .arg(
                Arg::with_name("name")
                .help("The container to search for")
                .required(true)
                .index(1)
            )
        )
        .get_matches();
    
    match matches.subcommand() {
        ("pull", Some(sub_m)) => {
            let image = sub_m.value_of("image").unwrap_or("none");
            pull::pull(image);
        },
        ("backup", Some(sub_m)) => {
            let container = sub_m.value_of("container").unwrap();
            backup::backup(container);
        },
        ("install", Some(sub_m)) => {
            let name = sub_m.value_of("name").unwrap();
            install::install(name);
        },
        ("list", Some(_)) => {
            list::list();
        },
        (_, _) => {
            println!("{}", "No subcommand was used");
        }
    };
}
