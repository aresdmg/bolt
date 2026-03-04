APP_NAME := bolt
INSTALL_DIR := $(HOME)/.local/bin
BIN := target/release/$(APP_NAME)

.PHONY: build release run clean install uninstall

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

clean:
	cargo clean

install: release
	mkdir -p $(INSTALL_DIR)
	cp $(BIN) $(INSTALL_DIR)/$(APP_NAME)
	@echo "Installed to $(INSTALL_DIR)/$(APP_NAME)"

uninstall:
	rm -f $(INSTALL_DIR)/$(APP_NAME)
	@echo "Removed $(INSTALL_DIR)/$(APP_NAME)"