#include <pci/pci.h>
#include <stdio.h>
#include <stdlib.h>


int main() {

    struct pci_access *pciaccess = pci_alloc();
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

    return 0;

}
