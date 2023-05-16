# vega
A simpler and faster command line system information tool designed to be faster than neofetch

![screenshot](https://github.com/onyasumi/vega/blob/master/screenshot.png?raw=true)

[![GPLv3](https://img.shields.io/badge/license-GPLv3-green)](#)

## DEPENDENCIES
**vega**'s dependencies are designed to minimal:

- GNU/Linux (or FreeBSD)
- POSIX sh
- libpci
- gcc (build)
- GNU Make (build)

Experimental FreeBSD support requires [procfs(5) to be mounted](https://man.freebsd.org/cgi/man.cgi?query=procfs&apropos=0&sektion=5&manpath=FreeBSD+14.0-CURRENT&arch=default&format=html). Note that GNU Make and BSD make are incompatible, so gmake is needed to build on FreeBSD.

## INSTALLATION

Install

    make install

Uninstall

    make remove


Also remove `~/.config/vega/config.sh` (`make remove` does not do this)

## CONFIGURATION

The main config file is located at `/etc/vega.d/config.sh`
Run `vega --installconf` to install a copy to your user config directory (this will override the systemwide config)

