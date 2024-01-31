//https://docs.rs/clap/latest/clap/_derive/_cookbook/git/index.html

use clap::{arg, Command};
mod gitf_commands;

fn cli() -> Command{
    Command::new("gitf")
        .about("CLI tool to manage git favourites.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("add")
            .about("Add branch to favourites")
            .arg_required_else_help(true)
            .arg(arg!(<BRANCH> "Branch to add"))
            )
        .subcommand(
            Command::new("remove")
            .about("Removing branch from favourites")
            .arg_required_else_help(true)
            .arg(arg!(<BRANCH> "Branch to remove"))
            )
        .subcommand(
            Command::new("branch")
            .about("See current branches in favourites")
            )
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand(){
        Some(("add", sub_matches)) => {
            let branch_name = sub_matches.get_one::<String>("BRANCH").expect("required");   
            gitf_commands::add(branch_name).expect("Error");
        },
        Some(("remove", sub_matches)) => {
            let branch_name = sub_matches.get_one::<String>("BRANCH").expect("required");
            gitf_commands::remove(branch_name).expect("Error");
        },
        Some(("branch", _sub_matches)) => {
            gitf_commands::view_branches().expect("Error");
        }
        _ => unreachable!()
    }
}
