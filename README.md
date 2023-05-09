# vega
A simpler and faster command line system information tool designed to be faster than neofetch

![screenshot](https://github.com/bloodandcoffee/vega/blob/master/screenshot.png?raw=true)

[![GPLv3](https://img.shields.io/badge/license-GPLv3-green)](#)

## DEPENDENCIES
**vega**'s dependencies are designed to minimal:

- GNU/Linux
- POSIX sh
- libpci
- gcc (build)

## INSTALLATION

Install

    make install

Uninstall

    make remove


Also remove `~/.config/vega/config.sh` (`make remove` does not do this)

## CONFIGURATION

The main config file is located at `/etc/vega.d/config.sh`
Run `vega --installconf` to install a copy to your user config directory (this will override the systemwide config)

