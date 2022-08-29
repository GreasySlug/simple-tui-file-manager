# simple-tui-file-manager

Simple? file manager to practice using tui-rs

## Installation

cargo buildのみ

```sh
git clone https://github.com/GreasySlug/simple-tui-file-manager.git
rustup update
cd simle-tui-file_manager
cargo install --path . # .cargo/bin/に保存される
```

## Getting Start

このアプリを起動すると`config.ron`ファイルが読み込まれます

しかし，config.ronが設定されていなければ，デフォルトのキーバインドが読み込まれます

そして，このアプリを起動したければ`sfm(simple file manager)`と入力してください

### 1. move to config directory

ここに書いてあるのはどのモードでも利用できるコマンドです

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
| q        | quick puit(from normal mode)
| S-i | input mode|
| S-v | stacker mode|
| esc | normal mode |

このテーブルを見ながら以下のパスまで移動してみましょう

```sh
# windows
C:\Users\user_name\AppData

# linux/ macOS
/home/user_name/.config/
```

### 2. make directory

`Shift`+`i`を同時押しすることでノーマルモードからインプットモードへ切り替えられます

もし変なところを押してしまった場合はとりあえず`Esc`を押してノーマルモードに戻りましょう

続いて，`Shift`+`m`を同時押しすることで，新しくディレクトリを作成します

入力した文字は上部に表示されます

simple_file_managerと入力しエンターを押しましょう

作成したsimple_file_managerへ移動します

### 3. make config.ron file

インプットモードままなら`Shift`+`i`を押すことで新しくファイルを作成できます

これも上部に入力した文字が表示されます

config.ronと入力しエンターを押しましょう

これでファイルが作成されたはずです

### 4. copy config.ron file

デフォルトの設定がREADME.mdにあるのでそれをコピーしておきましょう

次に，`Enter`を押すことでターミナルのエディタ(vim/vi)が起動します

もし起動しなかった場合には自分の使っているエディタで先ほど作ったconfig.ronを開いてください

エディタでペーストして保存し，終了します

### 5. Exit and confirm settings

ファイルマネージャの画面に戻ってきたら，`Esc`でノーマルモードに戻り，`q`でこのアプリを終了することができます

再度sfmと入力してテーマなどが変わるのを確認しましょう

## default Settings

キーマップやテーマなどを自由に変更することができます

カラーはRgb(0~255, 0~255, 0~255)で色を自分で設定することができます

キーマップは現在のところ2回のキーコンボのみをサポートしています

`g g g`のような3回のキーコンボはできません

`g g`までです

同時押しは`A-S-C-a`のように`-(ハイフン)`で区切ります

同時押しできるのは`A(Alt)`, `S(Shift)`, `C(Control)`のみです

Fキーと他のキーなどを同時押しは現在のところできません

日本語キーボード配列で`#_!?`などのShiftキーを押さないと入力できないキーはデフォルトでShiftの同時入力として判別されます

"#"のみで"S-3"などのように設定する必要はありません

```ron
(
    // colors: White, Black, Red, Green, Blue, Magenta, Cyan, Yellow, Gray, DarkGray
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
        editor: "nvim",
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