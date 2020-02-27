pub struct Repository {
    repository: git2::Repository,
    display_name: String,
}

impl Repository {
    pub fn new(repository: git2::Repository, display_name: String) -> Self {
        Repository {
            display_name,
            repository,
        }
    }

    pub fn name(&self) -> String {
        self.display_name.clone()
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
        .map(|p| {
            if show_absolute_path {
                p.to_owned()
            } else {
                p.chars().skip(base_path_len + 1).collect()
            }
        })
        .map(|p| p.trim_end_matches('/').to_owned())
}
