# Aizen Makefile — build, verify, install, and first-run environment setup.
#
#    make setup        # first-run env: toolchain check, .env scaffold, PATH hint
#    make build        # cargo build --release --bin aizen
#    make build-dense  # release build with the optional `dense` semantic tier
#    make check        # cargo check (fast feedback)
#    make test         # cargo test --bin aizen
#    make fmt / clippy # cargo fmt / cargo clippy
#    make lint         # fmt + clippy
#    make install      # build from source, copy into ~/.aizen/bin
#    make install-bin  # install the prebuilt GitHub release binary (no toolchain)
#    make run          # cargo run -- <args...>
#    make update       # aizen self-update (downloads latest release)
#    make clean        # cargo clean
#
# Apple Silicon note: `make` itself can run as an x86_64 (Rosetta) process on an
# M1/M2/M3. `uname -m` then reports x86_64 even though the hardware is arm64 and
# a native arm64 cargo is what we want to drive. On macOS we read the hardware
# directly (`sysctl hw.optional.arm64`) and, when running under Rosetta
# (`sysctl.proc_translated`), prefix every tool invocation with `arch -arm64` so
# builds and installs agree with the machine instead of the emulated shell.

.SUFFIXES:

SHELL := /bin/sh

AIZEN_INSTALL ?= $(HOME)/.aizen
BIN_DIR := $(AIZEN_INSTALL)/bin

UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

NATIVE :=
ifeq ($(UNAME_S),Darwin)
  HAS_ARM64 := $(shell sysctl -n hw.optional.arm64 2>/dev/null)
  ifeq ($(HAS_ARM64),1)
    TRANSLATED := $(shell sysctl -n sysctl.proc_translated 2>/dev/null)
    ifeq ($(TRANSLATED),1)
      NATIVE := arch -arm64
    endif
  endif
endif

.PHONY: help setup build build-dense check test fmt clippy lint install install-bin run update clean

help:
	@echo "Aizen targets:"
	@echo "  make setup        first-run environment setup (toolchain, .env, PATH)"
	@echo "  make build        release build (the shipped artifact)"
	@echo "  make build-dense  release build with the 'dense' semantic tier"
	@echo "  make check        cargo check (fast feedback)"
	@echo "  make test         cargo test --bin aizen"
	@echo "  make lint         cargo fmt + cargo clippy"
	@echo "  make install      build from source and install into $(BIN_DIR)"
	@echo "  make install-bin  install the prebuilt GitHub release (no toolchain)"
	@echo "  make run          cargo run"
	@echo "  make update       aizen self-update"
	@echo "  make clean        cargo clean"

setup:
	@echo "== Aizen environment setup (os=$(UNAME_S), uname -m=$(UNAME_M))"
ifeq ($(UNAME_S),Darwin)
	@echo "== Apple Silicon: hw.optional.arm64=$(HAS_ARM64) proc_translated=$(TRANSLATED)"
ifeq ($(TRANSLATED),1)
	@echo "!! make is running under Rosetta 2 (x86_64 emulation). Commands will be"
	@echo "!! wrapped with 'arch -arm64', but for a clean shell check Terminal >"
	@echo "!! Settings > General > 'Open using Rosetta' is OFF for this app."
endif
endif
	@command -v rustc >/dev/null 2>&1 && echo "== rustc : $$(rustc --version)" || { echo "!! rustc not found — install via https://rustup.rs"; }
	@command -v cargo >/dev/null 2>&1 && echo "== cargo : $$(cargo --version)" || { echo "!! cargo not found — install via https://rustup.rs"; }
	@if command -v cc >/dev/null 2>&1 && cc -v 2>&1 | grep -q "license agreements"; then \
		echo "!! 'cc' is broken: Xcode license not accepted. Run: sudo xcodebuild -license"; \
	fi
	@if [ ! -f .env ] && [ -f .env.example ]; then cp .env.example .env && echo "== created .env from .env.example (fill in AIZEN_BASE_URL / AIZEN_API_KEY)"; \
	else [ -f .env ] && echo "== .env already present"; fi
	@echo "== done. Install the binary with 'make install' (or 'make install-bin'),"
	@echo "   then run 'aizen config' once. Binary dir: $(BIN_DIR)"

build:
	$(NATIVE) cargo build --release --bin aizen

build-dense:
	$(NATIVE) cargo build --release --features dense --bin aizen

check:
	$(NATIVE) cargo check

test:
	$(NATIVE) cargo test --bin aizen

fmt:
	$(NATIVE) cargo fmt

clippy:
	$(NATIVE) cargo clippy --all-targets --all-features

lint: fmt clippy

install: build
	@mkdir -p "$(BIN_DIR)"
	@cp target/release/aizen "$(BIN_DIR)/aizen"
	@chmod +x "$(BIN_DIR)/aizen"
	@echo "aizen installed -> $(BIN_DIR)/aizen"
	@$(MAKE) --no-print-directory path-hint

install-bin:
	$(NATIVE) sh install.sh

run:
	$(NATIVE) cargo run -- $(ARGS)

update:
	$(NATIVE) "$(BIN_DIR)/aizen" update

clean:
	$(NATIVE) cargo clean

path-hint:
	@case ":$$PATH:" in *":$(BIN_DIR):"*) ;; *) \
		echo "Add aizen to your PATH (append to ~/.bashrc or ~/.zshrc):"; \
		echo "    export PATH=\"$(BIN_DIR):\$$PATH\""; \
		;; esac
	@echo "Then run:  aizen config"
