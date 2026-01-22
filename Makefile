# --------------------------------------
# Project
# --------------------------------------
APP_NAME := meaty_ore
ASSETS   := assets
DIST     := dist

# --------------------------------------
# Rust targets
# --------------------------------------
MAC_ARM   := aarch64-apple-darwin
MAC_INTEL := x86_64-apple-darwin
LINUX     := x86_64-unknown-linux-gnu
WINDOWS   := x86_64-pc-windows-msvc

# --------------------------------------
# Cargo
# --------------------------------------
CARGO  := cargo
RUSTUP := rustup

# --------------------------------------
# Build modes
# --------------------------------------
RELEASE_FLAGS :=
DEBUG_FLAGS   := --features debug

# --------------------------------------
# Default
# --------------------------------------
.PHONY: all
all: setup mac linux windows

# --------------------------------------
# Setup
# --------------------------------------
.PHONY: setup
setup:
	$(RUSTUP) target add \
		$(MAC_ARM) \
		$(MAC_INTEL) \
		$(LINUX) \
		$(WINDOWS)

# ======================================
# macOS builds
# ======================================
.PHONY: mac mac-debug mac-arm mac-intel mac-universal
.PHONY: mac-arm-debug mac-intel-debug

mac: mac-arm mac-intel
mac-debug: mac-arm-debug mac-intel-debug

# -------- macOS ARM --------
mac-arm:
	$(CARGO) build --release --target $(MAC_ARM) $(RELEASE_FLAGS)
	@$(MAKE) package-mac TARGET=$(MAC_ARM) SUFFIX=macos-arm

mac-arm-debug:
	$(CARGO) build --target $(MAC_ARM) $(DEBUG_FLAGS)
	@$(MAKE) package-mac TARGET=$(MAC_ARM) SUFFIX=macos-arm-debug PROFILE=debug

# -------- macOS Intel --------
mac-intel:
	$(CARGO) build --release --target $(MAC_INTEL) $(RELEASE_FLAGS)
	@$(MAKE) package-mac TARGET=$(MAC_INTEL) SUFFIX=macos-intel

mac-intel-debug:
	$(CARGO) build --target $(MAC_INTEL) $(DEBUG_FLAGS)
	@$(MAKE) package-mac TARGET=$(MAC_INTEL) SUFFIX=macos-intel-debug PROFILE=debug

# -------- Universal (release only) --------
mac-universal: mac-arm mac-intel
	@mkdir -p $(DIST)/universal/$(APP_NAME)
	lipo -create \
		target/$(MAC_ARM)/release/$(APP_NAME) \
		target/$(MAC_INTEL)/release/$(APP_NAME) \
		-output $(DIST)/universal/$(APP_NAME)/$(APP_NAME)
	@cp -R $(ASSETS) $(DIST)/universal/$(APP_NAME)/
	@tar -czf $(DIST)/$(APP_NAME)-macos-universal.tar.gz -C $(DIST)/universal $(APP_NAME)

# ======================================
# Linux builds
# ======================================
.PHONY: linux linux-debug

linux:
	$(CARGO) build --release --target $(LINUX) $(RELEASE_FLAGS)
	@$(MAKE) package-linux SUFFIX=linux-x86_64

linux-debug:
	$(CARGO) build --target $(LINUX) $(DEBUG_FLAGS)
	@$(MAKE) package-linux SUFFIX=linux-x86_64-debug PROFILE=debug

# ======================================
# Windows builds
# ======================================
.PHONY: windows windows-debug

windows:
	$(CARGO) build --release --target $(WINDOWS) $(RELEASE_FLAGS)
	@$(MAKE) package-windows SUFFIX=windows-x86_64

windows-debug:
	$(CARGO) build --target $(WINDOWS) $(DEBUG_FLAGS)
	@$(MAKE) package-windows SUFFIX=windows-x86_64-debug PROFILE=debug

# ======================================
# Packaging helpers
# ======================================
PROFILE ?= release

package-mac:
	@mkdir -p $(DIST)/$(TARGET)/$(APP_NAME)
	@cp -R $(ASSETS) $(DIST)/$(TARGET)/$(APP_NAME)/
	@cp target/$(TARGET)/$(PROFILE)/$(APP_NAME) $(DIST)/$(TARGET)/$(APP_NAME)/
	@tar -czf $(DIST)/$(APP_NAME)-$(SUFFIX).tar.gz -C $(DIST)/$(TARGET) $(APP_NAME)

package-linux:
	@mkdir -p $(DIST)/$(LINUX)/$(APP_NAME)
	@cp -R $(ASSETS) $(DIST)/$(LINUX)/$(APP_NAME)/
	@cp target/$(LINUX)/$(PROFILE)/$(APP_NAME) $(DIST)/$(LINUX)/$(APP_NAME)/
	@tar -czf $(DIST)/$(APP_NAME)-$(SUFFIX).tar.gz -C $(DIST)/$(LINUX) $(APP_NAME)

package-windows:
	@mkdir -p $(DIST)/$(WINDOWS)/$(APP_NAME)
	@cp -R $(ASSETS) $(DIST)/$(WINDOWS)/$(APP_NAME)/
	@cp target/$(WINDOWS)/$(PROFILE)/$(APP_NAME).exe $(DIST)/$(WINDOWS)/$(APP_NAME)/
	@cd $(DIST)/$(WINDOWS) && zip -r $(APP_NAME)-$(SUFFIX).zip $(APP_NAME)

# ======================================
# Utilities
# ======================================
.PHONY: clean run run-debug

clean:
	rm -rf target $(DIST)

run:
	$(CARGO) run

run-debug:
	$(CARGO) run $(DEBUG_FLAGS)