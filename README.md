# simple-tui-file-manager

Simple? file manager to practice using tui-rs

[Japanese ver](docs/README(jp).md)

## Installation

Only cargo build

```sh
git clone https://github.com/GreasySlug/simple-tui-file-manager.git
rustup update
cd simle-tui-file_manager
cargo build
```

## Getting Start

you can run this app by typing `sfm(simple file manager)`.

When you started this app, this app loads a `config.ron`.

No `config.ron` is set up, so the default keymap are loaded.

### 1. move to config directory

The following keymaps can be used in any mode.

| Key      | description     |
| -------  | --------------- |
| j, up    | move to up         |
| k, down  | move to down       |
| h, Left  | move to parent dir |
| l, Right | move to child dir  |
| g g | move to top of dir |
| S-g | move to bottom of dir |
| tab      | next dir tab    |
| S+tab    | prev dir tab    |
| q        | quick quit(from normal mode)
| S-i | input mode|
| S-v | stacker mode|
| esc | normal mode |

See this table and move to this path

```sh
# windows
C:\Users\user_name\AppData

# linux/ macOS
/home/user_name/.config/
```

### 2. make directory

Press `Shift`+`i` at the same time, then shift from normal mode to input mode.

Press `Shift`+`m` at the same time, and  make a directory.

If you mistype something, press `Esc` to return to normal mode.

The text to be your input is displayed at the top. Type "simple_file_manager" and press Enter.

Let't move to simple_file_manager directory

### 3. make config.ron file

If you are still in Input mode, `Shift`+`i` will create the file.

The text to be entered is displayed at the top.

Type `config.ron` and press Enter.

### 4. copy config.ron file

The default settings are in the README.md, so copy them.

Press `Enter`, then terminal editor(vim/ vi) is lunched.

So paste, save, and exit the editor.

### 5. Exit and confirm settings

When you come back to the file manager, you can exit with `q`.

Then type sfm again to see if the theme, etc. has changed!

## default Settings

```ron
(
    // colors: White, Black, Red, Green, Blue, Magenta, Cyan, Yello, Gray, DarkGray
    // LightRed, LightGreen, LightBlue, LightMagenta, LightCyan, LightYellow, Rgb(r,g,b)
    theme: SettingTheme (
        background: White, // main background
        header: Magenta, // file name, permission, size, date
        boarder: Black, // boarder lines
        directory: Blue, // dir font
        file_item: Black, // file font
        select: Green, // selecting highlight
        warning_background: Rgb(233, 163, 38), // orange
        warning_foreground: White,
        error_background: Rgb(233, 40, 30), // Red
        error_foreground: White,
                error: LightRed,
        command: [
            // bg, font
            (White, Black), // normal
            (White, Green), // input
            (White,  Blue), // stacker
            (White,  Red), // searcher
            ],
    ),
    symbols: {
        File: " ",
        Directory: "▶",
        Select: ">>",
    },
    user_settings: Settings (
        editor: "vim", // check your terminal editor
        show_hidden_files: true,
    ),
    additional_directories: [
        "Documents",
        "Downloads",
        "Desktop"
    ],
    user_keybindings: ModeKeybindings (
        normal : {
            "q" : "quit",
            "S-i": "input",
            "S-v": "stacker",

            "h": "move_to_parent_dir",
            "j": "move_to_next_file_item",
            "k": "move_to_prev_file_item",
            "l": "move_to_child_dir",
            "g g": "move_to_top_of_file_item",
            "S-g": "move_to_bottom_of_file_item",
            "tab": "next_dirtab",
            "S-tab": "prev_dirtab",
            "/": "search_file_items",
        },
        input: {
            "q q" : "quit",
            "esc": "normal",
            "S-v": "stacker",

            "h": "move_to_parent_dir",
            "j": "move_to_next_file_item",
            "k": "move_to_prev_file_item",
            "l": "move_to_child_dir",
            "tab": "next_dirtab",
            "S-tab": "prev_dirtab",

            "m": "make_directory",
            "i": "make_file",
            "enter": "edit",
        },
        stacker: {
            "q q" : "quit",
            "S-i": "input",
            "esc": "normal",

            "h":    "move_to_parent_dir",
            "j":    "move_to_next_file_item",
            "k":    "move_to_prev_file_item",
            "l":    "move_to_child_dir",
            "tab":  "next_dirtab",
            "S-tab": "prev_dirtab",

            "C-n": "stacker_next_file_item",
            "C-p": "stacker_prev_file_item",

            "s":   "stacker_toggle_select",
            "u":   "stacker_pop",
            "C-s": "stacker_select_all_recursively",
            "C-a": "stacker_select_all",
            "p":   "stacker_paste",
            "S-p": "stacker_stacking_paste",
            "m":   "stacker_move",
            "d d":   "stacker_delete", // use carefully, delete completely
            "S-d S-d": "stacker_delete_all", // use carefully, delete completely
        },
        searcher: {
            "j": "searcher_next_file_item",
            "k": "searcher_prev_file_item",
            "l": "searcher_move_to_child_dir",
            "C-a": "searcher_select_all",
            "esc": "normal",
        }
    )
)
```
All commands [here](docs/command_list.md)

## To Contribute

This is a project for practicing github and rust.

Please check this [doc](/docs/contribute.md)