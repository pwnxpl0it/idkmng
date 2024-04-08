# idkmng: A TOML based project initializer.

idkmng is a template-based command-line tool that helps to initialize projects with the necessary files and syntax.whatever you're working on, idkmng can set up your project with a single command.


Template-based, with easy [TOML](https://toml.io/en/) support!

## Idea 🧠:
In many cases you need to create a project that needs some special files,for example a browser extension, your worktree will look like:
```
manifest.json
Content.js
etc.. 
```

And each one of these files has its own syntactic sugar,
with idkmng you can use the command line to initialize your projects no matter what syntactic sugar needed!
for example a web extension:
```sh
$ idkmng browser_extension
```
and 💥 your files are ready for your initial commit, How amazing is that ? 
same for any other thing because it is template-based.

This gives you the ability to customize your projects initialization process as you want for example, you can have more than one template to create a browser extension but they are different in `manifest.json` or whatever.

## Installation
If you have Rust 🦀 🚀 installed on your machine run the following command:

```sh
$ cargo install --git https://www.github.com/pwnxpl0it/idkmng
```

<!-- TODO: Add templates repo-->

Alternatively you can go to [Releases](https://github.com/pwnxpl0it/idkmng/releases) and download the binary

```console
$ sudo tar -xzf idkmng-<RELEASE>.tar.gz -C /usr/local/bin
```

Replace <RELEASE> with the version number or tag of the release you want to install.

now you should be able to run `idkmng` in your terminal!

## Creating a template 📜
<!--There is a template for creating a template! 
it is located in templates directory/template.toml
just run the following command! 
```sh
$ idkmng new
```
enter template name and you should have one, it will go inside `~/.config/idkmng/templates/TEMPLATENAME.toml`
also you can edit that Template too to create you own template that creates a template 🎉,<br>

note that the template `info` section can be totally ignored, straight to the point where you only create files and directories you want!<br>
-->
The template structure is like the following:
```toml
[info]
name = ""
description = ""
author = ""

[[files]] # file1 
path="/path/to/file1"
content="""

"""

[[files]] # file2
path="/path/to/file2"
content="""

#etc...
"""
```

> [!TIP]
> Info section is not required and can be ignored

<!-- so it's super easy to write and you can get this structure using <br> ```$ idkmng new```. <br> -->

-  `path` represents the path of the file you want to save content into
-  `content` represents the content of the file,<br>
  and so on,you can have as much files as you want,<br>
- `path` and `content` can have keywords/placholders that idkmng knows it's value by default. Like `{{$HOME}}` or `{{$CURRENTDIR}}`.
- some of those default keywords may ask for user input like `{{$PROJECTNAME}}`. 

Here is a table of defualt keyword for idkmng:

| Keyword/placeholder   | Value     | Example          |
|--------------- | ---------------  | ---------------  |
| PROJECTNAME   |                   |                  |
| CURRENTDIR    | Current directory | pwd=/foo/bar => `bar`|
| HOME          | Home directory    | `/home/user/`    |
| YYYY    | Current Year in YYYY format| 2024    |
| YY | Current Year in YY format| 24    |
| MM | Current Month | 2 |
| DD | Current Day | 24 |
| NOW | Current date and time | `2024-02-23 22:22:38.151417626 +00:00` |
| NOW_UTC | Current date and time in UTC | `2024-02-23 22:21:17.897444668 UTC` |

you can have keywords/placeholder that asks for user input to take as a value by using the following format `{{$%s:f}}` this already works with `{{$PROJECTNAME}}`. but you can have your own...<br>
Example: 
```toml
# --snip
content="""
{{$TEST:read}}
"""
```

Functions supported by idkmng:

| Function   | Description    | Example  |
|--------------- | --------------- | ---------------  |
| read   | Asks for user input to replace placeholder with   | `{{$TEST:read}}` |
| env    | Replace with value from environment variables     | `{{$PATH:env}}` |

also keep in mind that once a function gets called on a keyword you can use `{{$TEST:read}}` or `{{$TEST}}` both are going to work and value will be replaced for both of them.

### Automated Template generation 🚀
Also there is one more time saving way! if you have some files in `/foo/bar/` you can just run `idkmng init` and it will create a template for you with directory name `bar.toml` and it will have all your files in it! 🌸

## Special Keywords 🔧
You can have your own Keywords for idkmng to replace with desired values!
Idkmng finds them stored in $HOME/.config/idkmng/config.toml
```toml
[Keywords]
AUTHOR = "Mohamed Tarek"
USERNAME = "@pwnxpl0it"
GITHUB = "https://github.com/pwnxpl0it"
#etc .....
```

## 👾 Neovim plugin (idkmng.nvim) 
I wrote a neovim plugin that makes it a way easier, Check it out [idkmng.nvim](https://www.github.com/pwnxpl0it/idkmng.nvim).
