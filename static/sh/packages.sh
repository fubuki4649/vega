#!/bin/sh
# Inspired by neofetch by Dylan Araps
has() { type "$1" 2>/dev/null 1>/dev/null ; }
show() { printf "$("$@" | wc -l | awk '{print $1}') ($(echo $1 | sed 's/-.*//g'))"; }

has kiss       && show kiss l
has cpt-list   && show cpt-list
has pacman-key && show pacman -Qq --color never
has dpkg       && show dpkg-query -f '.\n' -W
has xbps-query && show xbps-query -l
has apk        && show apk info
has opkg       && show opkg list-installed
has pacman-g2  && show pacman-g2 -Q
has lvu        && show lvu installed
has tce-status && show tce-status -i
has pkg_info   && show pkg_info
has pkgin      && show pkgin list
has tazpkg     && show tazpkg list
has sorcery    && show gaze installed
has alps       && show alps showinstalled
has butch      && show butch list
has swupd      && show swupd bundle-list --quiet
has pisi       && show pisi li
has pacstall   && show pacstall -L
has rpm        && show rpm -qa
has pkg        && show pkg info