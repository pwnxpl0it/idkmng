# idkmng
A TOML based project initializer.

Template-based, with easy [TOML](https://toml.io/en/) support!

## Idea 🧠:
In many cases you need to create a project that need some special files,for example a browser extension, your worktree will look like:
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
same for any other thing because it is template-based like: nvim plugins, browser extensions.

This gives you the ability to customize your projects initialization process as you want for example, you can have more than one template to create a browser extension but they are different in `manifest.json` or whatever.

💡 If you have many templates with long names and you're too lazy to type them I recommend using a fuzzy finder like fzf and use the following trick
```sh 
$ idkmng `ls ~/.config/idkmng/templates | fzf`
```
you can also create a binding for it in tmux for example, and if you're using Neovim scroll down to the bottom, I have a gift for you.

## Installation
If you have Rust 🦀 🚀 installed on your machine run the following command:

```sh
$ cargo install --git https://www.github.com/0xr00t3d/idkmng
```
<!--if not I don't know how you are still alive jk-->
<!-- TODO: Add templates repo-->
<!-- TODO: Add releases-->
now you should be able to run `Idkmng` in your terminal!

## Creating a template 📜
There is a template for creating a template! 
it is located in templates directory/template.toml
just run the following command! 
```sh
$ idkmng new
```
enter template name and you should have one, it will go inside `~/.config/idkmng/templates/TEMPLATENAME.toml`
also you can edit that Template too to create you own template that creates a template 🎉,
note that the template `info` section can be totally ignored, straight to the point where you only create files and directories you want!
The template structure is like the following:
```toml
[info] # Not required
name = ""
description = ""
author = ""

[[files]] # file1 
path="/path/to/file1"
content=""" # File content

"""

[[files]] # file2
path="/path/to/file2"
content=""" # File content

#etc...
"""
```
so it's super easy to write and you can get this structure using `idkmng new`.
`path` represents the path of the file you want to save content into, and `content` represents the content of the file,
and so on,you can have as much files as you want,
`path` and `content` can have `$PROJECTNAME`, `$CURRENTDIR` `$HOME` for now as a special *"Keywords"* for idkmng, as it is going to ask you the Project name or Automatically add current directory if `$CURRENTDIR` is provided!,same with `$HOME`.

### Automated Template generation 🚀
Also there is one more time saving way! if you have some files in `/foo/bar/` you can just run `idkmng init` and it will create a template for you with directory name `bar.toml` and it will have all your files in it! 🌸

## Neovim plugin(idkmng.nvim)
I wrote a neovim plugin that makes it a way easier, Check it out [idkmng.nvim](https://www.github.com/0xr00t3d/idkmng.nvim).
