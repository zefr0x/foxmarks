# foxmarks

[![release](https://github.com/zefr0x/foxmarks/actions/workflows/release.yml/badge.svg)](https://github.com/zefr0x/foxmarks/actions/workflows/release.yml)

<br>

[![AUR version](https://img.shields.io/aur/version/foxmarks?label=AUR)](https://aur.archlinux.org/packages/foxmarks)
[![AUR votes](https://img.shields.io/aur/votes/foxmarks?label=AUR%20votes)](https://aur.archlinux.org/packages/foxmarks)

A blazingly fast, highly extendable and easily integrated command line **read-only** interface for [firefox](https://www.mozilla.org/firefox)'s bookmarks and history.

## Features
- 🧾 Free software under the [GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html) licence.
- 🔗 Built with the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy) in mind.
- 💪 Written in the [Rust](https://www.rust-lang.org/) programming language.
- 🚫 windows is not supported.

## Installation

### Download Binary From Github
For every new release a Github workflow will build a binary in Github servers and will upload it as a release asset in Github releases.

You can find the latest Github release [here](https://github.com/zefr0x/foxmarks/releases/latest) or the releases page [here](https://github.com/zefr0x/foxmarks/releases).

### [AUR](https://aur.archlinux.org/packages/foxmarks)

[![AUR last modified](https://img.shields.io/aur/last-modified/foxmarks)](https://aur.archlinux.org/cgit/aur.git/log/?h=foxmarks)

#### Using yay
```shell
yay -Sa foxmarks
```

#### Using paru
```shell
paru -Sa foxmarks
```

## Build
> [!Note]
> You need to have [`cargo`](https://doc.rust-lang.org/cargo/) installed in you system.

```shell
git clone https://github.com/zefr0x/foxmarks.git

cd foxmarks

# Checkout to a release tag e.g. v1.0.1
git checkout vx.x.x

cargo build --release
```
You will find the binary in `./target/release/foxmarks`


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
foxmarks -t Esr bookmarks
```

> Release: firefox-release
>
> Esr: firefox-esr
>
> Dev: firefox-dev

> If you are using a custom profile as your default one or you are using the flatpak version of firefox, then you need to use `profile-path` option to specify the profile you want to read from.

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
        while read -r line;
        do
            IFS=";"
            read -r title url <<< $line
            printf "%s\0icon\x1fbookmarks\x1fmeta\x1f%s\x1finfo\x1f%s\x1f\n" $title $url $url
        done
    }

elif [[ $ROFI_RETV = 1 ]];
then
    xdg-open $ROFI_INFO
fi
```

> [!Warning]
> You might use another language like Python for better and faster implementaion.

## Config
The default options might not fit your needs, so you can change them via a config file.

The config file is `$XDG_CONFIG_HOME/foxmarks/config` or by default `~/.config/foxmarks/config`.
Just create it and specify the options you need with the ini format:
```ini
[database]
firefox_type = Release
firefox_home_path = ~/.mozilla/firefox/
profile_path = xxxxxxxx.default-release

[output]
column_delimiter = |
row_delimiter = ;
```
