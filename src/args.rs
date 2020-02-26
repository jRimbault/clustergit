use crate::build_info;

use clap::{App, Arg, ArgMatches};
use std::ffi::OsString;

pub enum Argument {
    Directory,
    AbsolutePath,
}

static DESCRIPTION: &str = "\
will scan through all subdirectories looking for a .git directory. \
When it finds one it'll look to see if there are any changes and let you know. \
If there are no changes it can also push and pull to/from a remote location.";

pub fn parse<T, I>(itr: I) -> ArgMatches<'static>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    use Argument::*;
    let long_version = format!(
        "{} {}, {}",
        build_info::PKG_VERSION,
        build_info::GIT_VERSION.unwrap_or("dirty"),
        build_info::RUSTC_VERSION,
    );
    let about = format!("{} {}", build_info::PKG_NAME, DESCRIPTION);
    App::new(build_info::PKG_NAME)
        .version(build_info::PKG_VERSION)
        .long_version(long_version.as_str())
        .about(about.as_str())
        .author(build_info::PKG_AUTHORS)
        .arg(
            Arg::with_name(Directory.as_str())
                .help(Directory.description())
                .required(true),
        )
        .arg(
            Arg::with_name(AbsolutePath.as_str())
                .help(AbsolutePath.description())
                .long(AbsolutePath.as_str())
                .short(AbsolutePath.as_str().to_uppercase()),
        )
        .get_matches_from(itr)
}

impl Argument {
    pub fn as_str(&self) -> &'static str {
        use Argument::*;
        match *self {
            Directory => "directory",
            AbsolutePath => "absolute",
        }
    }

    pub fn description(&self) -> &'static str {
        use Argument::*;
        match *self {
            Directory => "directory to parse sub dirs from",
            AbsolutePath => "show absolute paths",
        }
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(self.as_str())
    }
}

impl std::convert::AsRef<str> for Argument {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
