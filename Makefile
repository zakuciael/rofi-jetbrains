PKGNAME := rofi-jetbrains
LIB_NAME := librofi_jetbrains.so
PLUGIN_NAME := jetbrains.so

CARGO ?= cargo
CARGO_TARGET_DIR ?= target
CARGO_RELEASE_DIR ?= $(CARGO_TARGET_DIR)/release

PLUGINS_DIR_CONFIG = $(shell pkg-config --variable pluginsdir rofi)
PLUGINS_DIR ?= $(if $(PLUGINS_DIR_CONFIG),$(PLUGINS_DIR_CONFIG),lib/rofi)
PLUGIN_INSTALL_PATH := "$(PLUGINS_DIR)/$(PLUGIN_NAME)"
LICENSE_DIR ?= /usr/share/licenses/$(PKGNAME)

build:
	cargo build --release --lib

clean:
	cargo clean

install:
	# Install plugin
	install -DT "$(CARGO_RELEASE_DIR)/$(LIB_NAME)" "$(DESTDIR)$(PLUGIN_INSTALL_PATH)"

	# Copy license file
	install -Dt $(DESTDIR)$(LICENSE_DIR) LICENSE

uninstall:
	rm ${PLUGIN_INSTALL_PATH}
	rm -rf ${LICENSE_DIR}