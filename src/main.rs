#![forbid(unsafe_code)]

mod args;
mod build_info;
mod repository;

use clap::ArgMatches;
use colored::*;
use repository::Repository;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
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
    let repositories = find_git_repositories(base_path, show, base_path_len)?;
    let max_padding = repositories
        .iter()
        .map(|r| r.name().len())
        .fold(None, |max, cur| match max {
            None => Some(cur),
            Some(x) => Some(if cur > x { cur } else { x }),
        })
        .unwrap_or(0);

    if args.is_present(ShowBranch) {
        for repository in repositories {
            println!(
                "{repository:<width$} : {status}",
                repository = repository.name(),
                width = max_padding,
                status = repository_branch(&repository),
            );
        }
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

fn repository_branch(repository: &Repository) -> ColoredString {
    match repository.branch() {
        Ok(branch) => branch.green(),
        Err(ref e) if e.code() == git2::ErrorCode::UnbornBranch => {
            "not on a branch".to_owned().dimmed()
        }
        Err(_) => "unknown".to_owned().dimmed(),
    }
}

fn find_git_repositories<P>(
    path: P,
    show_absolute_path: bool,
    path_len: usize,
) -> io::Result<Vec<Repository>>
where
    P: AsRef<Path>,
{
    Ok(
        globwalk::GlobWalkerBuilder::from_patterns(path, &["**/.git/"])
            .build()?
            .filter_map(Result::ok)
            .filter_map(|dir| git2::Repository::open(dir.into_path()).ok())
            .map(|r| {
                let display_name = repository::workdir_path(&r, show_absolute_path, path_len);
                Repository::new(r, display_name.unwrap_or("".to_owned()))
            })
            .collect(),
    )
}
