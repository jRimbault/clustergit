use crate::repository::Repository;
use colored::*;

pub fn status(repository: &Repository, text: String) -> ColoredString {
    let mut messages = Vec::new();
    let mut clean = true;

    if !text.contains("On branch master") && !text.contains("On branch develop") {
        let branch = repository.branch().unwrap_or("".to_string());
        messages.push(format!("On branch {}", branch).blue());
    }

    if text.contains("nothing added to commit but untracked files present") {
        messages.push("Untracked files".yellow());
        clean = false;
    }

    if text.contains("Changes not staged for commit") {
        messages.push("Changes".bright_red());
        clean = false;
    }

    if text.contains("Your branch is ahead of") {
        messages.push("Unpushed commits".red());
    }

    if clean {
        messages.insert(0, "Clean".green());
    }

    messages
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(", ")
        .normal()
}
