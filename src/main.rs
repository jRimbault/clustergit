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
    let base_path = fs::canonicalize(
        args.value_of(Directory)
            .expect("Arg should have been defined by clap"),
    )?;
    let base_path = base_path.to_str().unwrap();
    let repositories = fshelper::find_git_repositories(base_path)?;
    let repositories = repositories
        .iter()
        .filter_map(|p| p.to_str())
        .collect::<Vec<&str>>();

    let mapper = RepositoriesMapper::new(
        args.is_present(AbsolutePath),
        base_path.len(),
        repositories
            .iter()
            .map(|r| r.len())
            .fold(
                None,
                find(|prev, curr| if prev < curr { curr } else { prev }),
            )
            .unwrap_or(0),
    );

    let result = execute_task(args, mapper, &repositories).join("\n");
    println!("{}", result);
    Ok(())
}

fn execute_task(
    args: ArgMatches<'static>,
    mapper: RepositoriesMapper,
    repositories: &[&str],
) -> Vec<String> {
    use args::Argument::*;
    if args.is_present(ShowBranch) {
        mapper.map(&repositories, repository_branch)
    } else if args.is_present(GitStatus) {
        mapper.map(&repositories, repository_status)
    } else if args.is_present(GitFetch) {
        mapper.map(&repositories, repository_fetch)
    } else if args.is_present(GitPull) {
        mapper.map(&repositories, repository_pull)
    } else if args.is_present(GitPush) {
        mapper.map(&repositories, repository_push)
    } else {
        repositories
            .iter()
            .map(|p| p.to_owned())
            .map(repository::cut_path(
                args.is_present(AbsolutePath),
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

fn find<T, S>(selector: S) -> impl Fn(Option<T>, T) -> Option<T>
where
    S: Fn(T, T) -> T,
{
    move |prev, curr| match prev {
        None => Some(curr),
        Some(previous_item) => Some(selector(previous_item, curr)),
    }
}
