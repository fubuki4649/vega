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

#include <sys/sysinfo.h>
#include <sys/statvfs.h>
#include <sys/utsname.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <pci/pci.h>
#include <ifaddrs.h>
#include <netdb.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

int main(int argc, char** argv) {

    fprintf(stderr, "vegautils is a part of vega. Do not run vegautils directly. Doing so may result in undefined behaviour.\n");
    fprintf(stderr, "Run 'vega --help\' for details.\n");

    struct sysinfo system;
    struct statvfs disk;
    struct pci_access *pciaccess;
    struct ifaddrs* ifaddress;

    if(argc == 1) goto exit;

    if(!strcmp(argv[1], "--disk")) goto disk;
    else if(!strcmp(argv[1], "--gpu")) goto gpu;
    else if(!strcmp(argv[1], "--kernel")) goto kernel;
    else if(!strcmp(argv[1], "--uptime")) goto uptime;
    else if(!strcmp(argv[1], "--ip")) goto localip;
    else goto exit;

disk: ;
    statvfs("/", &disk);

    #define DISK_USED_GB llroundl((disk.f_blocks - disk.f_bfree) * disk.f_frsize / 1073741824.0)
    #define DISK_TOTAL_GB llroundl((disk.f_blocks) * disk.f_frsize / 1073741824.0)
    printf("%ldGB / %ldGB\n", DISK_USED_GB, DISK_TOTAL_GB);
    goto exit;

gpu: ;
    pciaccess = pci_alloc();
    pci_init(pciaccess);            // Initialize the PCI library
    pci_scan_bus(pciaccess);        // Gets list of PCI devices

    struct pci_dev *device = pciaccess->devices;
    for(; device; device=device->next){
        pci_fill_info(device, PCI_FILL_IDENT | PCI_FILL_BASES | PCI_FILL_CLASS);    // Reads PCI info
        
        // Checks if PCI device is a display controller
        if((device->device_class & 0b1111111100000000) == 0x0300){
            char *buffer = malloc(128);
            pci_lookup_name(pciaccess, buffer, 128, PCI_LOOKUP_VENDOR | PCI_LOOKUP_DEVICE, device->vendor_id, device->device_id);

            printf("%s [%x:%x]\n", buffer, device->vendor_id, device->device_id);
            free(buffer);

            break;
        }
    }

    if(!device) printf("None\n");
    pci_cleanup(pciaccess);
    goto exit;

kernel: ;
    struct utsname os;
    uname(&os);

    printf("%s %s\n", os.sysname, os.release);
    goto exit;

uptime: ;
    sysinfo(&system);

    long uptime = system.uptime;
    long prntSeconds = uptime % 60;
    long prntMinutes = (uptime / 60) % 60;
    long prntHours = (uptime / 3600) % 24;
    long prntDays = uptime / 86400;

    if(prntDays) {
        printf("%ld day%s ", prntDays, (prntDays != 1) ? "s" : "");
        prntSeconds = 0;
    }
    if(prntHours) printf("%ld hour%s ", prntHours, (prntHours != 1) ? "s" : "");
    if(prntMinutes) printf("%ld minute%s ", prntMinutes, (prntMinutes != 1) ? "s" : "");
    if(prntSeconds) printf("%ld second%s ", prntSeconds, (prntSeconds != 1) ? "s" : "");

    printf("\n");
    goto exit;

localip: ;
    getifaddrs(&ifaddress);

    char* addr = malloc(16);
    bool hasConnection = false;

    for(struct ifaddrs* i = ifaddress; i; i = i->ifa_next) {
        if(i->ifa_addr->sa_family != AF_INET) continue;     // Ignores the wrong type of address
        getnameinfo(i->ifa_addr, sizeof(struct sockaddr_in), addr, NI_MAXHOST, NULL, 0, NI_NUMERICHOST);
        
	    if(!strncmp(addr, "127", 3) || !strncmp(addr, "169", 3) || !strncmp(addr, ":", 1)) continue;    // Ignore loopback and localhost

        // Check for local IPv4
        if(!strncmp(addr, "192", 3) || !strncmp(addr, "172", 3) || !strncmp(addr, "10", 2)){
            printf("%s\n", addr);
            hasConnection = true;
            break;
        }
    }

    if(!hasConnection) printf("No Connection\n");

    free(addr);
    freeifaddrs(ifaddress);
    goto exit;

exit: ;
    return 0;

}
