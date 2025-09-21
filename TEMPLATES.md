# Template Documentation

This file provides the documentation for .projup files and about templates in general.

## Folder Structure
All files in all folders, apart from .projup, are copied into the project folder.
A file named ".projup" must be placed in the root directory of the template. The template will case errors if it placed anywhere else.
All files, including non-text files, will be passed through the string substitution and file names can optionally be as well.
Non-text files will likely through errors and stop the operation as they will not be correctly converted into a utf8 string.
Due to this, it is advisable not to include any non-text files in a template.

## .projup File Syntax
The .projup syntax consists of tags, sets and variable references.
Each new line defines a new syntax line, so new line characters cannot be specified in property values.
Each line is also trimmed of whitespace, so leading and trailing spaces also cannot be specified in property values.
Other than between `""`, spaces are ignored unless escaped by a `\`.

A tag consists of its name surrounded by square brackets, e.g. `[template]`.

Comments can be added by starting a line with `//`. Note that comments cannot be defined after the first character in a line.

Everything else is either a declaration or property set.
A set is one "object" followed by an `=` operator, with the rest of the line being the value.
If there is more than one `=`, the first non escaped one is used if possible, otherwise it is a declaration.
A declaration is the same as after the `=` in a set.

Objects are defined as being one of an Absolute, a string or a variable.
- Absolutes are just the plain text parsed without whitespace, and multiple space separated words and symbols are counted as one object.
In absolutes, a `\` can be used to escape whitespace, `=`, `\`, `"` or `$` characters, allowing them to be in included.
- A string is just content between `""`. Within strings, only `"` and `\` need to be escaped to be included.
- A variable is referenced by a `$` immediately followed (no spaces) by an alphanumeric word, allowing `_`.
Variables can also be followed by a `:` and then a string which is passed as the variables' format, e.g. `$name:"pascal"`

## .projup Files
The .projup file must include a name set under a `[template]` tag.
Note that variable references cannot be used within the template secion.
Within the template secion:
- version can be set, which must be a maximum of three parts dot separated. Defaults to 1.0.0.
- file_names can be set to either true or false, defaults to false.
This specifies whether file names are parsed through the string substitution as well as files.

A `[subs]` tag can also be defined with sets underneath specifying the string substitutions to use when copying files.
The left side of the `=` specifies the string to search for, and the right side specifies the string to replace.
The right side can include variable references, so that sections of the template files can customised upon creation e.g. by name.

A `[deps]` tag can be defined as well with sets underneath to specify git submodules.
The left side of the `=` specifies the folder location of the submodule, relative to the root directory of the project.
The right side is the submodule source url that git will use to get the submodule.

There are three auto defined variables when loading a template:
- `$name` is the name project being created.
Formats for name can control the casing convention used, which are (case insensitive):
`"camel"`, `"pascal"`, `"snake"`, `"macro"`, `"camel_snake"`, `"pascal_snake"`, `"kebab"`, `"cobol"`, `"train"`, `"title"` and `"sentence"`.
- `$date` is the date at the time of creating with default formatting of `"%d/%m/%Y"`.
Formatting for date follows the rust formatting specified in [this documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).
- `$time` is the time at which the project is created with default formatting of `"%H:%M:%S"`.
Formatting for time follows the rust formatting specified in [this documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

Other variables can be referenced, but these will have to be defined by the user upon project creation.
Note that there are no errors thrown when the formatting of a variable is not valid. In such cases, the default formatting is applied.

## Example
Here is an example .projup file:
```
[template]
name = zene
version = 1
file_names = true

[subs]
README_NAME = $name
LICENCE_YEAR = $date:"%Y"
[[name]] = $name:"Pascal"
[[ud]] = $use_double

[deps]
./deps/Structs = https://github.com/Me222282/ZeneStructs.git
./deps/Graphics = https://github.com/Me222282/ZeneGraphics.git
./deps/Windowing = https://github.com/Me222282/ZeneWindowing.git
```