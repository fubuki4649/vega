#!/bin/sh

# Fail fast on unset variables or pipeline errors
set -eu

# Helper: command exists?
have()   { command -v "$1" >/dev/null 2>&1; }

# Helper: add “N (label)” to one-line buffer
append() { [ -n "${out-}" ] && out="$out, $1" || out="$1"; }

# Helper: run cmd, count lines, append if >0
count_cmd() {
    label=$1; shift
    n=$(
        LC_ALL=C "$@" 2>/dev/null | wc -l | tr -d '[:space:]' || echo 0
    )
    [ "${n:-0}" -gt 0 ] && append "$n ($label)"
}

##############################################################################
# mainstream managers
##############################################################################
have kiss        && count_cmd kiss            kiss l
have cpt-list    && count_cmd cpt             cpt-list
have pacman      && count_cmd pacman          pacman -Qq --color never
have dpkg-query  && count_cmd dpkg            sh -c 'dpkg-query -f . -W | wc -c'
have xbps-query  && count_cmd xbps            xbps-query -l
have apk         && count_cmd apk             apk info
have opkg        && count_cmd opkg            opkg list-installed
have pacman-g2   && count_cmd pacman-g2       pacman-g2 -Q
have lvu         && count_cmd lvu             lvu installed
have tce-status  && count_cmd tce             tce-status -i
have pkg_info    && count_cmd pkg_info        pkg_info
have pkgin       && count_cmd pkgin           pkgin list
have tazpkg      && count_cmd tazpkg          tazpkg list
have gaze        && count_cmd sorcery         gaze installed
have alps        && count_cmd alps            alps showinstalled
have butch       && count_cmd butch           butch list
have swupd       && count_cmd swupd           swupd bundle-list --quiet
have pisi        && count_cmd pisi            pisi li
have pacstall    && count_cmd pacstall        pacstall -L
have pkg         && count_cmd freebsd-pkg     pkg info

##############################################################################
# rpm / dnf  (use sqlite cache when possible)
##############################################################################
if have dnf && have sqlite3 && [ -f /var/cache/dnf/packages.db ]; then
    n=$(sqlite3 /var/cache/dnf/packages.db 'SELECT count(pkg) FROM installed' 2>/dev/null || echo 0)
    [ "$n" -gt 0 ] && append "$n (dnf)"
elif have rpm; then
    count_cmd rpm rpm -qa
fi

##############################################################################
# Portage / Gentoo
##############################################################################
if [ -d /var/db/pkg ]; then
    n=$(find /var/db/pkg -type d -name '*-*' 2>/dev/null \
        | wc -l | tr -d '[:space:]')
    [ "$n" -gt 0 ] && append "$n (emerge)"
fi

##############################################################################
# Flatpak
##############################################################################
have flatpak && count_cmd flatpak flatpak list

##############################################################################
# Nix
##############################################################################
if have nix-store; then
    [ -e /run/current-system/sw ] && \
        count_cmd nix-system  nix-store -qR /run/current-system/sw
    [ -e "$HOME/.nix-profile" ] && \
        count_cmd nix-user    nix-store -qR "$HOME/.nix-profile"
    [ -e /nix/var/nix/profiles/default ] && \
        count_cmd nix-default nix-store -qR /nix/var/nix/profiles/default
fi

##############################################################################
# Final output
##############################################################################
[ -n "${out-}" ] && printf '%s\n' "$out" || echo "None"
