#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <sys/statvfs.h>


int main() {

    struct statvfs disk;
    statvfs("/", &disk);

    #define DISK_USED_GB llroundl((disk.f_blocks - disk.f_bfree) * disk.f_frsize / 1073741824.0)
    #define DISK_TOTAL_GB llroundl((disk.f_blocks) * disk.f_frsize / 1073741824.0)
    printf("%ldGB / %ldGB\n", DISK_USED_GB, DISK_TOTAL_GB);

    return 0;

}
