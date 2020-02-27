#![forbid(unsafe_code)]

mod args;
mod build_info;

use clap::ArgMatches;
use colored::*;
use git2::Repository;
// use rayon::prelude::*;
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
    let repositories = find_git_repositories(base_path)?;

    let repositories: Vec<_> = repositories
        .iter()
        .map(|r| (r.display_path(show, base_path_len), r))
        .collect();
    let max_padding = repositories
        .iter()
        .map(|r| r.0.len())
        .fold(None, |max, cur| match max {
            None => Some(cur),
            Some(x) => Some(if cur > x { cur } else { x }),
        })
        .unwrap_or(0);

    if args.is_present(ShowBranch) {
        for repository in repositories {
            println!(
                "{repository:<width$} : {status}",
                repository = repository.0,
                width = max_padding,
                status = repository
                    .1
                    .current_branch()
                    .unwrap_or("HEAD".to_owned())
                    .green(),
            );
        }
    } else {
        println!(
            "{}",
            repositories
                .iter()
                .map(|r| r.0.to_owned())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    Ok(())
}

fn find_git_repositories<P: AsRef<Path>>(path: P) -> io::Result<Vec<Repository>> {
    Ok(
        globwalk::GlobWalkerBuilder::from_patterns(path, &["**/.git/"])
            .build()?
            .filter_map(Result::ok)
            .filter_map(|dir| Repository::open(dir.into_path()).ok())
            .collect(),
    )
}

trait RepositoryExt {
    fn display_path(&self, show: bool, base_path_len: usize) -> String;
    fn current_branch(&self) -> Option<String>;
}

impl RepositoryExt for Repository {
    fn display_path(&self, show: bool, base_path_len: usize) -> String {
        self.workdir()
            .and_then(|p| p.to_str())
            .map(|p| {
                if show {
                    p.to_owned()
                } else {
                    p.chars().skip(base_path_len + 1).collect()
                }
            })
            .map(|p| p.trim_end_matches('/').to_owned())
            .unwrap_or("".to_owned())
    }

    fn current_branch(&self) -> Option<String> {
        use git2::ErrorCode;
        let head = match self.head() {
            Ok(head) => Some(head),
            Err(ref e)
                if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound =>
            {
                None
            }
            Err(_) => None,
        };
        head.as_ref()
            .and_then(|h| h.shorthand())
            .map(|h| h.to_owned())
    }
}
