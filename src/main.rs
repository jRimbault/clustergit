#![forbid(unsafe_code)]

mod args;
mod build_info;

use clap::ArgMatches;
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

    let results: Vec<(String, git2::RepositoryState)> = repositories
        .iter()
        .map(|r| (r.display_path(show, base_path_len), r.state()))
        .collect();
    let max_len = results
        .iter()
        .map(|r| r.0.len())
        .fold(None, |max, cur| match max {
            None => Some(cur),
            Some(x) => Some(if cur > x { cur } else { x }),
        })
        .unwrap_or(0);

    for repository in results {
        println!(
            "{repository:<width$} : {status:?}",
            repository = repository.0,
            status = repository.1,
            width = max_len,
        );
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
}
