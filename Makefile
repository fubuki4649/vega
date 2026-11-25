# Makefile
CARGO := cargo
BINARY_NAME := vega
INSTALL_DIR := /usr/local/bin

.PHONY: all build install remove clean

all: build

build:
	$(CARGO) build --release

install: build
	sudo install -d $(INSTALL_DIR)
	sudo install -m 755 target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)

remove:
	sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)

clean:
	$(CARGO) clean

