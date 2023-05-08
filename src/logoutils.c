/*
This file is a part of vega

vega is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

vega is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
 
You should have received a copy of the GNU General Public License
along with vega. If not, see <https://www.gnu.org/licenses/>.
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <regex.h>

#define RED "\033[0;31m"
#define GREEN "\033[0;32m"
#define YELLOW "\033[0;33m"
#define BLUE "\033[0;34m"
#define MAGENTA "\033[0;35m"
#define CYAN "\033[0;36m"

#define BOLD "\033[1m"
#define SOFT "\033[2m"
#define RESET "\033[0m"


struct logo {
    int height, width;
    char* logo[];
};

#define ARCH_C1 RESET CYAN SOFT
#define ARCH_C2 RESET CYAN BOLD
const struct logo arch = {
    19, 39,
    {
        ARCH_C1 "                   -`" RESET,
        ARCH_C1 "                  .o+`" RESET,
        ARCH_C1 "                 `ooo/" RESET,
        ARCH_C1 "                `+oooo:" RESET,
        ARCH_C1 "               `+oooooo:" RESET,
        ARCH_C1 "               -+oooooo+:" RESET,
        ARCH_C1 "             `/:-:++oooo+:" RESET,
        ARCH_C1 "            `/++++/+++++++:" RESET,
        ARCH_C1 "           `/++++++++++++++:" RESET,
        ARCH_C1 "          `/+++o" ARCH_C2 "oooooooooo" ARCH_C1 "oo/`" RESET,
        ARCH_C2 "         " ARCH_C1 "./" ARCH_C2 "ooosssso++osssssso" ARCH_C1 "+`" RESET,
        ARCH_C2 "        .oossssso-````/ossssss+`" RESET,
        ARCH_C2 "       -osssssso.      :ssssssso." RESET,
        ARCH_C2 "      :osssssss/        osssso+++." RESET,
        ARCH_C2 "     /ossssssss/        +ssssooo/-" RESET,
        ARCH_C2 "   `/ossssso+/:-        -:/+osssso+-" RESET,
        ARCH_C2 "  `+sso+:-`                 `.-/+oso:" RESET,
        ARCH_C2 " `++:.                           `-/+/" RESET,
        ARCH_C2 ".`                                   `/" RESET
    }
};


#define DEB_C1 RESET RED BOLD
#define DEB_C2 RESET SOFT
const struct logo debian = {
    17, 27,
    {
        DEB_C1 "       _,met$$$$$gg." RESET,
        DEB_C1 "    ,g$$$$$$$$$$$$$$$P." RESET,
        DEB_C1 "  ,g$$P\"        \"\"\"Y$$.\"." RESET,
        DEB_C1 " ,$$P'              `$$$." RESET,
        DEB_C1 "',$$P       ,ggs.     `$$b:" RESET,
        DEB_C1 "`d$$'     ,$P\"'   " DEB_C2 "." DEB_C1 "    $$$" RESET,
        DEB_C1 " $$P      d$'     " DEB_C1 "     $$P" RESET,
        DEB_C1 " $$:      $$.   " DEB_C2 "-" DEB_C1 "    ,d$$'" RESET,
        DEB_C1 " $$;      Y$b._   _,d$P'" RESET,
        DEB_C1 " Y$$.    " DEB_C2 "`." DEB_C1 "`\"Y$$$$P\"'" RESET,
        DEB_C1 " `$$b      " DEB_C2 "\"-.__" RESET,
        DEB_C1 "  `Y$$" RESET,
        DEB_C1 "   `Y$$." RESET,
        DEB_C1 "     `$$b." RESET,
        DEB_C1 "       `Y$$b." RESET,
        DEB_C1 "          `\"Y$b._" RESET,
        DEB_C1 "              `\"\"\"\"" RESET
    }
};


#define UBUNTU_C1 RESET RED BOLD
#define UBUNTU_C2 RESET SOFT
const struct logo ubuntu = {
    20, 40,
    {
        UBUNTU_C1 "            .-/+oossssoo+-." RESET,
        UBUNTU_C1 "        ´:+ssssssssssssssssss+:`" RESET,
        UBUNTU_C1 "      -+ssssssssssssssssssyyssss+-" RESET,
        UBUNTU_C1 "    .ossssssssssssssssss" UBUNTU_C2 "dMMMNy" UBUNTU_C1 "sssso." RESET,
        UBUNTU_C1 "   /sssssssssss" UBUNTU_C2 "hdmmNNmmyNMMMMh" UBUNTU_C1 "ssssss\\" RESET,
        UBUNTU_C1 "  +sssssssss" UBUNTU_C2 "hm" UBUNTU_C1 "yd" UBUNTU_C2 "MMMMMMMNddddy" UBUNTU_C1 "ssssssss+" RESET,
        UBUNTU_C1 " /ssssssss" UBUNTU_C2 "hNMMM" UBUNTU_C1 "yh" UBUNTU_C2 "hyyyyhmNMMMNh" UBUNTU_C1 "ssssssss\\" RESET,
        UBUNTU_C1 ".ssssssss" UBUNTU_C2 "dMMMNh" UBUNTU_C1 "ssssssssss" UBUNTU_C2 "hNMMMd" UBUNTU_C1 "ssssssss." RESET,
        UBUNTU_C1 "+ssss" UBUNTU_C2 "hhhyNMMNy" UBUNTU_C1 "ssssssssssss" UBUNTU_C2 "yNMMMy" UBUNTU_C1 "sssssss+" RESET,
        UBUNTU_C1 "oss" UBUNTU_C2 "yNMMMNyMMh" UBUNTU_C1 "ssssssssssssss" UBUNTU_C2 "hmmmh" UBUNTU_C1 "ssssssso" RESET,
        UBUNTU_C1 "oss" UBUNTU_C2 "yNMMMNyMMh" UBUNTU_C1 "sssssssssssssshmmmh" UBUNTU_C1 "ssssssso" RESET,
        UBUNTU_C1 "+ssss" UBUNTU_C2 "hhhyNMMNy" UBUNTU_C1 "ssssssssssss" UBUNTU_C2 "yNMMMy" UBUNTU_C1 "sssssss+" RESET,
        UBUNTU_C1 ".ssssssss" UBUNTU_C2 "dMMMNh" UBUNTU_C1 "ssssssssss" UBUNTU_C2 "hNMMMd" UBUNTU_C1 "ssssssss." RESET,
        UBUNTU_C1 " \\ssssssss" UBUNTU_C2 "hNMMM" UBUNTU_C1 "yh" UBUNTU_C2 "hyyyyhdNMMMNh" UBUNTU_C1 "ssssssss/" RESET,
        UBUNTU_C1 "  +sssssssss" UBUNTU_C2 "dm" UBUNTU_C1 "yd" UBUNTU_C2 "MMMMMMMMddddy" UBUNTU_C1 "ssssssss+" RESET,
        UBUNTU_C1 "   \\sssssssssss" UBUNTU_C2 "hdmNNNNmyNMMMMh" UBUNTU_C1 "ssssss/" RESET,
        UBUNTU_C1 "    .ossssssssssssssssss" UBUNTU_C2 "dMMMNy" UBUNTU_C1 "sssso." RESET,
        UBUNTU_C1 "      -+sssssssssssssssss" UBUNTU_C2 "yyy" UBUNTU_C1 "ssss+-" RESET,
        UBUNTU_C1 "        \\`:+ssssssssssssssssss+:`" RESET,
        UBUNTU_C1 "            .-\\+oossssoo+/-." RESET
    }
};


#define FEDORA_C1 RESET BLUE SOFT
#define FEDORA_C2 RESET BLUE BOLD
#define FEDORA_C3 RESET SOFT
const struct logo fedora = {
    19, 38,
    {
        FEDORA_C1 "             .',;::::;,'." RESET,
        FEDORA_C1 "         .';:cccccccccccc:;,." RESET,
        FEDORA_C1 "      .;cccccccccccccccccccccc;." RESET,
        FEDORA_C1 "    .:cccccccccccccccccccccccccc:." RESET,
        FEDORA_C1 "  .;ccccccccccccc;" FEDORA_C3 ".:dddl" FEDORA_C2 ":." FEDORA_C1 ";ccccccc;." RESET,
        FEDORA_C1 " .:ccccccccccccc;" FEDORA_C3 "OWMKOOX" FEDORA_C2 "MWd" FEDORA_C1 ";ccccccc:." RESET,
        FEDORA_C1 ".:ccccccccccccc;" FEDORA_C3 "KMMc" FEDORA_C1 ";cc;" FEDORA_C2 "xMMc" FEDORA_C1 ";ccccccc:." RESET,
        FEDORA_C1 ",cccccccccccccc;" FEDORA_C3 "MMM." FEDORA_C1 ";cc;" FEDORA_C2 ";WW:" FEDORA_C1 ";cccccccc,",
        FEDORA_C1 ":cccccccccccccc;" FEDORA_C3 "MMM." FEDORA_C1 ";cc" FEDORA_C2 "xMMd" FEDORA_C1 "cccccccccc:" RESET,
        FEDORA_C1 ":ccccccc;" FEDORA_C2 "oxOO" FEDORA_C3 "O00MMM000" FEDORA_C2 "00kd" FEDORA_C1 ";cccccccccc:" RESET,
        FEDORA_C1 "cccccc;" FEDORA_C2 "0MMKxd" FEDORA_C3 "dddMMMkdd" FEDORA_C2 "cw" FEDORA_C1 ";cccccccccccc;" RESET,
        FEDORA_C1 "ccccc;" FEDORA_C2 "XM0\'" FEDORA_C1 ";cccc;" FEDORA_C3 "MMM." FEDORA_C1 ";cccccccccccccccc'" RESET,
        FEDORA_C1 "ccccc;" FEDORA_C2 "MMo" FEDORA_C1 ";ccccc;" FEDORA_C3 "MMW." FEDORA_C1 ";ccccccccccccccc;" RESET,
        FEDORA_C1 "ccccc;" FEDORA_C2 "0MNc." FEDORA_C1 "ccc" FEDORA_C3 ".xMMd" FEDORA_C1 ";ccccccccccccccc;" RESET,
        FEDORA_C1 "cccccc;" FEDORA_C2 "dNMW" FEDORA_C3 "XXXWM0:" FEDORA_C1 ";cccccccccccccc:," RESET,
        FEDORA_C1 "cccccccc;" FEDORA_C2 ".:" FEDORA_C3 "odl:." FEDORA_C1 ";cccccccccccccc:,." RESET,
        FEDORA_C1 ":cccccccccccccccccccccccccccc:'." RESET,
        FEDORA_C1 "\\cccccccccccccccccccccc:;,.." RESET,
        FEDORA_C1 "  w::cccccccccccccc::;,." RESET
    }
};


int main(int argc, char** argv) {

    fprintf(stderr, "vegautils is a part of vega. Do not run logoutils directly. Doing so may result in undefined behaviour.\n");
    fprintf(stderr, "Run 'vega --help' for details.\n");

    const struct logo* l = NULL;
    int ln = -1;

    if(argc < 3) goto exit;
    else ln = atoi(argv[2]);

    if(!strcmp(argv[1], "arch")) goto arch;
    else if(!strcmp(argv[1], "debian")) goto debian;
    else if(!strcmp(argv[1], "ubuntu")) goto ubuntu;
    else if(!strcmp(argv[1], "fedora")) goto fedora;
    else goto exit;

arch:
    l = &arch;
    goto print;

debian:
    l = &debian;
    goto print;

ubuntu:
    l = &ubuntu;
    goto print;

fedora:
    l = &fedora;
    goto print;

print:
    if(!strcmp(argv[2], "--width")) {
        printf("%d\n", l->width);
        goto exit;
    }

    // Pad with spaces
    if(ln >= l->height) {
        for(int i = 0; i < l->width; i++) printf(" ");
        goto exit;
    }

    //Print Logo + Padding
    printf("%s", l->logo[ln]);

    goto exit;

exit: ;
    if(l == NULL || ln == -1) return 1;
    else if(ln >= l->height) return 1;
    else return 0;

}
