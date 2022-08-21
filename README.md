# foxmarks
A blazingly fast, highly extendable and easily integrated command line **read-only** interface for [firefox](https://www.mozilla.org/firefox)'s bookmarks and history.

## Features
- 🧾 Free software under the [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html) licence.
- 🔗 Built with the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) in mind.
- 💪 Written in the [Rust](https://www.rust-lang.org/) programming language.
- 🚫 windows is not supported.

## Installation

## Usage
It's simple, for a list of your bookmarks run:
```shell
foxmarks bookmarks
```
For the browsing history:
```shell
foxmarks history
```
By default it will try to fetch the data from the default profile of `firefox-release`, but if you have `firefox-esr` or `firefox-dev` installed, you can specify a type:
```shell
foxmarks -t 1 bookmarks
```

> 0: firefox-release
> 1: firefox-esr
> 2: firefox-dev

For more options and details read the long help:
```shell
foxmarks --help
```

### Usage Examples
Since it's a simple tool that do one thing will, you should consider compining it with another tools.

Pipe the output to the [`column`](https://linuxhint.com/use-linux-column-command/) command to display the results in columns, so you can read them easily.
```shell
foxmarks bookmarks | column -t -s ";"
```
You can pipe them again to the [`less`](https://en.wikipedia.org/wiki/Less_(Unix)) command:
```shell
foxmarks bookmarks | column -t -s ";" | less
```

You can integrate the tool with your favorate launcher, for example here is a script for the [rofi launcher](https://github.com/davatorium/rofi):
```bash
#!/usr/bin/bash

if [[ $ROFI_RETV = 0 ]];
then
    foxmarks bookmarks | {
        while IFS= read -r line;
        do
            read title < <(echo $line | cut -f1 -d ";")
            read url < <(echo $line | cut -f2 -d ";")

            echo -en "$title\0icon\x1fbookmarks\x1fmeta\x1f$url\x1finfo\x1f$url\x1f\n"
        done
    }

elif [[ $ROFI_RETV = 1 ]];
then
    xdg-open $ROFI_INFO
fi
```

## Config
The default options might not fit your needs, so you can change them via a config file.

The config file is `$XDG_CONFIG_HOME/foxmarks/config` or by default `~/.config/foxmarks/config`.
Just create it and specify the options you need with the ini format:
```ini
[database]
firefox_type = 0
profile_id = xxxxxxxx.default-release

[output]
column_delimiter = |
row_delimiter = ;
```
