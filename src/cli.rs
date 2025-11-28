use crate::utils::common::create_printer;
use crate::utils::git::{fetch_prune, get_branches_to_delete, is_git_repo};
use clap::{Arg, ArgAction, Command};
use std::process::{Command as ProcessCommand, Stdio};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn build_cli() -> Command {
    Command::new("git-purge")
        .version(VERSION)
        .about("Purge stale local branches deleted from remote")
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .help("Force delete branches")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry_run")
                .short('n')
                .long("dry-run")
                .help("Show what would be deleted without deleting")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show detailed output")
                .action(ArgAction::SetTrue),
        )
}

pub fn run() {
    let matches = build_cli().get_matches();

    let force = matches.get_flag("force");
    let dry_run = matches.get_flag("dry_run");
    let verbose = matches.get_flag("verbose");

    let print = create_printer(verbose);

    is_git_repo();

    print("Fetching and pruning remote branches...\n");

    fetch_prune(verbose);

    let output = ProcessCommand::new("git")
        .args([
            "for-each-ref",
            "--format=%(refname:short) %(upstream:track)",
            "refs/heads",
        ])
        .output()
        .expect("Failed to run git for-each-ref");

    let output_string = String::from_utf8_lossy(&output.stdout).to_string();

    let branches_to_delete: Vec<String> = get_branches_to_delete(output_string);

    if branches_to_delete.is_empty() {
        print("No stale local branches found to delete.");
        std::process::exit(0);
    }

    if dry_run || verbose {
        if dry_run {
            println!("Would delete the following branches (dry-run):");
        } else {
            println!("The following stale local branches will be safely deleted:");
        }
        for branch in &branches_to_delete {
            println!("  - {}", branch);
        }
    }

    if !dry_run {
        for branch in &branches_to_delete {
            let flag = if force { "-D" } else { "-d" };
            let mut cmd = ProcessCommand::new("git");
            cmd.args(["branch", flag, branch]);
            if !verbose {
                cmd.stdout(Stdio::null()).stderr(Stdio::null());
            }
            let _ = cmd.status();
        }
    }
}
