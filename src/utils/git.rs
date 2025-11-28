use std::process::{Command, Stdio};

pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn fetch_prune(verbose: bool) {
    let _ = Command::new("git")
        .args(["fetch", "-p"])
        .stdout(if verbose {
            Stdio::inherit()
        } else {
            Stdio::null()
        })
        .stderr(if verbose {
            Stdio::inherit()
        } else {
            Stdio::null()
        })
        .status();
}

pub fn get_branches_to_delete(output: String) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 && parts[1] == "[gone]" {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect()
}
