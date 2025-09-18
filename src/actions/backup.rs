use projup::{error::{IntoProjUpError, ProjUpError}, file};

use crate::git;

use super::{load_backups, BACKUP_REMOTE};

pub fn backup() -> Result<(), ProjUpError>
{
    let file = match file::get_projects_path()
    {
        Some(f) => f,
        None => return Err(ProjUpError::ProgramFolder)
    };
    file::ensure_path(file.parent()).projup(&file)?;
    
    let b = load_backups(&file)?;
    
    for project in b.iter()
    {
        // let repo = Repository::open_ext::<&Path, &Path, [&Path; 0]>(project, RepositoryOpenFlags::CROSS_FS, [])?;
        // // let repo = Repository::open(&project)?;
        // let mut remote = repo.find_remote(BACKUP_REMOTE)?;
        // // find all branches
        // let mut branch_names = Vec::new();
        // for b in repo.branches(Some(BranchType::Local))?
        // {
        //     let b = b?;
        //     let reference = b.0.into_reference();
        //     let opt = reference.name();
        //     let bn = opt.ok_or(ProjUpError::UtfString)?;
        //     // create branch name string
        //     let mut str = String::with_capacity(bn.len() + 1);
        //     // + for force push
        //     str.push('+');
        //     str.push_str(bn);
        //     branch_names.push(str);
        // }
        // // push to backup remote
        // remote.push(&branch_names, None)?;
        
        git::run(git::GitOperation::Push { force: true, remote: BACKUP_REMOTE }, project)?;
    }
    
    return Ok(());
}