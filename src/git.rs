use std::{path::Path, process::Command};

use projup::error::{IntoProjUpError, ProjUpError};

pub enum GitOperation<'a>
{
    Init{
        bare: bool
    },
    Push{
        force: bool,
        remote: &'a str
    },
    SubmoduleAdd{
        url: &'a str,
        path: &'a Path
    },
    RemoteAdd{
        name: &'a str,
        url: &'a str
    },
    RemoteSet{
        name: &'a str,
        url: &'a str
    }
}

pub fn run<P>(opertaion: GitOperation, directory: P) -> Result<(), ProjUpError>
    where P: AsRef<Path>
{
    let mut git = Command::new("git");
    git.current_dir(directory);
    match opertaion
    {
        GitOperation::Init { bare } =>
        {
            git.arg("init");
            git.arg("-b");
            git.arg("main");
            if bare
            {
                git.arg("--bare");
            }
        },
        GitOperation::Push { force, remote } =>
        {
            git.arg("push");
            git.arg("--all");
            if force
            {
                git.arg("--force");
            }
            git.arg(remote);
        },
        GitOperation::SubmoduleAdd { url, path } =>
        {
            git.arg("submodule");
            git.arg("add");
            git.arg(url);
            git.arg(path);
        },
        GitOperation::RemoteAdd { name, url } =>
        {
            git.arg("remote");
            git.arg("add");
            git.arg(name);
            git.arg(url);
        },
        GitOperation::RemoteSet { name, url } =>
        {
            git.arg("remote");
            git.arg("set-url");
            git.arg(name);
            git.arg(url);
        }
    }
    let out = git.output().projup("")?;
    if out.status.success()
    {
        return Ok(());
    }
    
    let str = String::from_utf8(out.stderr).unwrap_or("".to_string());
    return Err(ProjUpError::GitError(str));
}