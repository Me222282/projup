# ProjUp

A simple project management system with template and backup features.

## Usage

**[--help | -h]** exists for all commands and displays a summary of the commands options.

Commands:
- backup
- clone
- config
- ls
- move
- new
- new-existing
- remove
- templates


### Backup
The backup command actually backs up the listed projects to their respective backup repositories.
This command also creates any missing backup repositories and adds their remotes if a project was created without access to the backup location.
This command fails if the backup location cannot be accessed at the time.
```
projup backup [--force | -f]
```

**`[--force | -f]`** has the same meaning as force in the **`new`** command.
This only applies if there are project backups being created by this command.


### Clone
The clone command calls git clone with the source url being a directory in the backup location.
The name does not need to exist in the registry, just the backup git repository (if it has been removed with **`--soft`**).
```
projup clone <name> [<path>]
```

**`<name>`** is the name of the project that is to be cloned.
This must be identical to the folder name within the backup location.

**`[<path>]`** is an optional target directory for the cloned repository.

### Config
The config command is for setting the template and backup locations.
This must be called at least once with a backup location for any other command to work.
```
projup config [(--template-location | -t) <path>] [(--backup-location | -b) <path>] [--soft | -s]
```

**`[(--template-location | -t) <path>]`** sets the template search directory to **`<path>`**.
Note that all subfolders 1 level deep are searched for templates.
This command does not actually perform the template search.

**`[(--backup-location | -b) <path>]`** sets the project backup directory to **`<path>`**.
This will cause all projects who's backup exists to change their remote to the new backup location.
Note that new backups are not created with this command for projects without a backup.

**`[--soft | -s]`** causes the path to be set for projup without moving the contents within the directories.
If this argument is not set, the directory contents will moved (or copied if necessary).


### Ls
The ls command lists all the projects' names and locations that are recorded in the registry.
```
projup ls
```


### Move
The move command is for changing a projects directory location, or renaming a project.
This command fails if the backup location cannot be accessed at the time.
Warning, this command does not check the new location of the project and could override files.
```
projup move <source> <destination> [--force | -f]
```

**`<source>`** is the original path to the root directory of the project that is going to be moved.
This must be a project that exists in the backup registry

**`<destination>`** is the path to the new location of the project's root directory.
If the name of the trailing folders differs, the project will be renamed.
If a project with the new name already exists in the registry, the command will fail.

**`[--force | -f]`** specifies that when renaming the backup, it should override any folder with the project's new name in the backup location.
If a folder with the new project name exists without specifying this argument, the operation will fail.
This argument does nothing if the backup location cannot be accessed at the time or the project's name does not change.


### New
The new command is for creating new projects, loading a template and adding it to the backup registry.
If the backup location cannot be accessed, the project is still created and added to the registry,
just the backup and git remote are not added until the **`backup`** command is called.
When creating the backup, a remote called "local-backup" is added to the repository which becomes the backup location.
Note that multiple backed up projects cannot have the same name, even if they are in different folders or drives.
```
projup new <path> [(--template | -t) <template>] [--force | -f] [-D <variables>..]
```
**`<path>`** is a path to the new project's root directory, where the trailing folder is the name of the project.
This is what gets passed to the *$name* variable in the .projup file.

**`[(--template | -t) \<template>]`** specifies an optional template to load into the project directory.
The template name must match that which is specified in the .projup file. Templates are researched if the template has not been recorded yet.
Note that the project is still created if the template loading fails in any way.

**`[--force | -f]`** specifies that when creating the backup, it should override any folder with the project's name in the backup location.
If a folder with the given name exists without specifying this argument, the operation will fail.
This argument does nothing if the backup location cannot be accessed at the time.

**`[-D <variables>..]`** specify extra variables to be passed to the template .projup file.
The command will fail if a template uses a variable that is not defined in this list.
This argument does nothing if a template is not given.


### New-existing
This is similar to **`new`**, except that it is for adding projects that have already been created.
The same backup rules apply as when calling **`new`**.
As projup creates a git remote, the command will fail if a remote called "local-backup" already exists for the repository.
```
projup new-existing <path> [--backup | -b] [--force | -f]
```
**`<path>`** is a path to the new project's root directory that must already exist with a git repository initialised.

**`[--backup | -b]`** causes a git push to the backup of the project to be called straight away.

**`[--force | -f]`** see description in **`new`** command.


### Remove
The remove command removes a project from the registry and deletes its backup.
It does NOT remove the project's original directory.
Note this command can be called whether the project's original directory exists or not.
```
projup remove <name> [--soft | -s]
```

**`<name>`** is the case sensitive name of the project in the registry to be deleted.

**`[--soft | -s]`** specifies that the backup repository is not deleted when the project is removed.


### Templates
The templates command is for managing the templates.
Without specifying a specific query, the command loads all templates found in the template directory and adds them to the known list.
When doing this it does not run full formatting and error checking on the templates .projup files.
Note that templates are renamed so that the folder is the same as the name specified in .projup.
If this fails, the template is not added to the list.
```
projup templates [--list | -l] [(--query | -q) <template>]
```

**`[--list | -l]`** specifies that the names of found templates are outputted to the console.

**`[(--query | -q) <template>]`** changes the command to query specifically the specified template name.
It then fully checks the formatting of .projup, displaying errors.
It also outputs all variables needed by the template and what formatting they request.
Note that a config never fails if the formatting of a variable is invalid.
