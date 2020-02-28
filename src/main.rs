#![forbid(unsafe_code)]

mod args;
mod build_info;
mod fshelper;
mod repository;

use clap::ArgMatches;
use colored::*;
use repository::Repository;
use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    process::exit(match run(args::parse(env::args())) {
        Ok(_) => exitcode::OK,
        Err(error) => {
            println!("{}", error);
            error.raw_os_error().unwrap_or(exitcode::SOFTWARE)
        }
    })
}

fn run(args: ArgMatches<'static>) -> io::Result<()> {
    use args::Argument::*;
    let absolute_path = fs::canonicalize(args.value_of(Directory).unwrap())?;
    let base_path = absolute_path.to_str().unwrap();
    let base_path_len = base_path.len();
    let show = args.is_present(AbsolutePath);
    let repositories = fshelper::find_git_repositories(base_path, show, base_path_len)?;
    let max_padding = repositories
        .iter()
        .map(|r| r.name().len())
        .fold(None, |max, cur| match max {
            None => Some(cur),
            Some(x) => Some(if cur > x { cur } else { x }),
        })
        .unwrap_or(0);

    if args.is_present(ShowBranch) {
        print_repositories(&repositories, max_padding, repository_branch);
    } else if args.is_present(GitStatus) {
        print_repositories(&repositories, max_padding, repository_status);
    } else if args.is_present(GitFetch) {
        print_repositories(&repositories, max_padding, repository_fetch);
    } else if args.is_present(GitPull) {
        print_repositories(&repositories, max_padding, repository_pull);
    } else if args.is_present(GitPush) {
        print_repositories(&repositories, max_padding, repository_push);
    } else {
        println!(
            "{}",
            repositories
                .iter()
                .map(|r| r.name())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    Ok(())
}

fn print_repositories<F>(repositories: &[Repository], max_padding: usize, info_getter: F)
where
    F: Fn(&Repository) -> ColoredString,
{
    for repository in repositories {
        println!(
            "{repository:<width$} : {info}",
            repository = repository.name(),
            width = max_padding,
            info = info_getter(&repository),
        );
    }
}

fn repository_branch(repository: &Repository) -> ColoredString {
    match repository.branch() {
        Ok(branch) => branch.green(),
        Err(ref e) if e.code() == git2::ErrorCode::UnbornBranch => {
            "not on a branch".to_owned().dimmed()
        }
        Err(_) => "unknown".to_owned().dimmed(),
    }
}

fn repository_status(_repository: &Repository) -> ColoredString {
    "not implemented yet".to_owned().red()
}

fn repository_fetch(_repository: &Repository) -> ColoredString {
    "not implemented yet".to_owned().red()
}

fn repository_pull(_repository: &Repository) -> ColoredString {
    "not implemented yet".to_owned().red()
}

fn repository_push(_repository: &Repository) -> ColoredString {
    "not implemented yet".to_owned().red()
}
