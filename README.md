# tino

TINO (Todo-Ideas-Notes-Organizer), A terminal application (TUI) to organize todos, ideas and notes written in markdown.

> [!IMPORTANT]
> This software was just test in Ubuntu as OS, and Wezterm as Terminal Emulator.

## Why the crate is called tinao

The crate tino exist and I didn't check before 🙃, my fault,
I decided that tinao will be fine because I and a TINO directory
called _academic notes_, so it is because of that.

The project, and binary, will continue be called tino because
that was the main idea.

> [!NOTE]
> I want to make the academic note directory optional. 

## Installation

> [!IMPORTANT]
> Rust is need to install this TUI.
> [Install Rust](https://rust-lang.org/tools/install/)

To install run:
``` bash
cargo install tinao
```

## Configuration
1. Create a .tino.toml file on your config file:
    Linux: `/home/<username>/.config`
    Windows: `C:\Users\<username>\AppData\Roaming`
    MacOS: `/Users/<username>/Library/Application Support`

> [!WARNING]
> Only Linux has tested, Windows will be tested.
> I will thanks if someone can test it on MacOS,
> because I don't have a machine with MacOS.

2. Add your custom path for each TINO directory.

Example:
```toml
[tino_dirs]
todos_dir = "~/todos"
ideas_dir = "~/cool-ideas"
notes_dir = "~/random-things"
academic_notes_dir = "~/school"
```

3. Run `tino` and your are good to go.

## Key bindings

- Tab: Jump between elements.
- Up and Down, or j and k, for vim/nvim user,: Scroll through types, categories, TINO files and file preview.
- Enter on File name element to create file.
- Enter on file from TINO files list to open editor.
> [!WARNING]
> tino use $EDITOR env var or vim command if the env var isn't set.
> This feature doesn't work in Windows yet.
- v on a file from TINO files list to preview file content in File preview element.
- Ctrl+n: Go to File name element.
- Ctrl+t: Go to Type element.
- Ctrl+c: Go to PARA category element.
- Ctrl+l: Go to TINO files list.
- Ctrl+p: Go to File preview element.

## License

Copyright (c) LuisanaMTDev <luisanamartineztorres25@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
