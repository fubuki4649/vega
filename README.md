# vega
A simpler and faster command line system information tool designed to be faster than neofetch

![screenshot](https://github.com/onyasumi/vega/blob/master/screenshot.png?raw=true)

[![GPLv3](https://img.shields.io/badge/license-GPLv3-green)](#)

## DEPENDENCIES
**vega**'s dependencies are designed to minimal:

- GNU/Linux
- POSIX sh
- libpci
- gcc (build)

Experimental FreeBSD support requires [procfs(5) to be mounted](https://man.freebsd.org/cgi/man.cgi?query=procfs&apropos=0&sektion=5&manpath=FreeBSD+14.0-CURRENT&arch=default&format=html). It is recommended to install with `INSTALLPATH=/usr/local/etc/vega.d` since FreeBSD expects user-installed programs to go in `/usr/local`.

## INSTALLATION

Install

    make install

Uninstall

    make remove


Also remove `~/.config/vega/config.sh` (`make remove` does not do this)

## CONFIGURATION

The main config file is located at `/etc/vega.d/config.sh`
Run `vega --installconf` to install a copy to your user config directory (this will override the systemwide config)

