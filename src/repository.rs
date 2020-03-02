use std::path::Path;

pub struct Repository {
    repository: git2::Repository,
}

impl Repository {
    pub fn new(repository: git2::Repository) -> Self {
        Repository { repository }
    }

    pub fn open<P>(path: P) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        let r = git2::Repository::open(path).ok();
        match r {
            Some(repo) => Some(Repository::new(repo)),
            None => None,
        }
    }

    pub fn name(&self, show_absolute_path: bool, base_path_len: usize) -> String {
        workdir_path(&self.repository, show_absolute_path, base_path_len).unwrap_or("".to_owned())
    }

    pub fn branch(&self) -> Result<String, git2::Error> {
        self.repository.current_branch()
    }
}

trait RepositoryExt {
    fn current_branch(&self) -> Result<String, git2::Error>;
}

impl RepositoryExt for git2::Repository {
    fn current_branch(&self) -> Result<String, git2::Error> {
        match self.head() {
            Ok(head) => Ok(head
                .shorthand()
                .map(|h| h.to_owned())
                .unwrap_or("".to_owned())),
            Err(e) => Err(e),
        }
    }
}

pub fn workdir_path(
    repository: &git2::Repository,
    show_absolute_path: bool,
    base_path_len: usize,
) -> Option<String> {
    repository
        .workdir()
        .and_then(|p| p.to_str())
        .map(cut_path(show_absolute_path, base_path_len))
}

pub fn cut_path(show_absolute_path: bool, base_path_len: usize) -> impl Fn(&str) -> String {
    move |path| {
        if show_absolute_path {
            path.to_owned()
        } else {
            path.chars().skip(base_path_len + 1).collect()
        }
        .trim_end_matches('/')
        .to_owned()
    }
}
