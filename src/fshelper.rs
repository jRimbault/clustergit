use std::io;
use std::path::Path;

pub fn find_git_repositories<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    Ok(
        globwalk::GlobWalkerBuilder::from_patterns(path, &["**/.git/"])
            .build()?
            .filter_map(Result::ok)
            .filter_map(|dir| git2::Repository::open(dir.into_path()).ok())
            .filter_map(|repo| repo.workdir().map(|p| p.to_path_buf()))
            .filter_map(|repo| repo.to_str().map(|s| s.to_owned()))
            .collect(),
    )
}
