use crate::repository::{self, Repository};

use std::io;
use std::path::Path;

pub fn find_git_repositories<P>(
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
                Repository::new(r, &display_name.unwrap_or("".to_owned()))
            })
            .collect(),
    )
}
