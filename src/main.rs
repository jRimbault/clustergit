#![forbid(unsafe_code)]

mod args;
mod build_info;
mod fshelper;
mod repository;

use args::Argument;
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
    let base_path = fs::canonicalize(
        args.value_of(Argument::Directory)
            .expect("Arg should have been defined by clap"),
    )
    .expect("Directory should be a valid path");
    let base_path = base_path.to_str().unwrap();
    let repositories = fshelper::find_git_repositories(&base_path)?;
    // make strings immutable
    let repositories: Vec<&str> = repositories.iter().map(|s| s.as_ref()).collect();

    let mapper = RepositoriesMapper::new(
        args.is_present(Argument::AbsolutePath),
        base_path.len(),
        repositories
            .iter()
            .max_by_key(|r| r.len())
            .map(|r| r.len())
            .unwrap_or(0),
    );

    let results_shown = execute_task(args, mapper, &repositories).join("\n");
    println!("{}", results_shown);
    Ok(())
}

fn execute_task(
    args: ArgMatches<'static>,
    mapper: RepositoriesMapper,
    repositories: &[&str],
) -> Vec<String> {
    if args.is_present(Argument::ShowBranch) {
        mapper.map(&repositories, repository_branch)
    } else if args.is_present(Argument::GitStatus) {
        mapper.map(&repositories, repository_status)
    } else if args.is_present(Argument::GitFetch) {
        mapper.map(&repositories, repository_fetch)
    } else if args.is_present(Argument::GitPull) {
        mapper.map(&repositories, repository_pull)
    } else if args.is_present(Argument::GitPush) {
        mapper.map(&repositories, repository_push)
    } else {
        repositories
            .iter()
            .map(|p| p.to_owned())
            .map(repository::cut_path(
                args.is_present(Argument::AbsolutePath),
                mapper.base_path_len,
            ))
            .collect()
    }
}

struct RepositoriesMapper {
    show_absolute_path: bool,
    base_path_len: usize,
    max_padding: usize,
}

impl RepositoriesMapper {
    fn new(show_absolute_path: bool, base_path_len: usize, max_padding: usize) -> Self {
        RepositoriesMapper {
            show_absolute_path,
            base_path_len,
            max_padding: max_padding - if show_absolute_path { 0 } else { base_path_len },
        }
    }

    fn map<F>(&self, repositories: &[&str], info_getter: F) -> Vec<String>
    where
        F: Fn(&Repository) -> ColoredString,
        F: Send + Sync,
    {
        use rayon::prelude::*;
        repositories
            .par_iter()
            .filter_map(|r| Repository::open(r))
            .map(|repository| {
                format!(
                    "{repository:<width$}: {info}",
                    repository = repository.name(self.show_absolute_path, self.base_path_len),
                    width = self.max_padding,
                    info = info_getter(&repository),
                )
            })
            .collect()
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
