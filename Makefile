# Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)
#
# This file is part of FearInit and is dual-licensed:
#   1. AGPL-3.0-or-later — see LICENSE for the full text.
#   2. Commercial license — see COMMERCIAL.md for details.
#
# --- Config ---
# Linux x86_64 (Standard Intel/AMD Linux - static compilation)
# Linux ARM64 (e.g., Raspberry Pi 4/5, AWS Graviton servers)
# Linux RISC-V (64-bit RISC-V devices)
# Linux None (Your Own Kernel)
TARGETS = x86_64-unknown-linux-musl aarch64-unknown-linux-musl riscv64gc-unknown-linux-musl
#		  x86_64-unknown-linux-none aarch64-unknown-linux-none riscv64gc-unknown-linux-none
RUSTFLAGS_RELEASE = -C link-arg=-Wl,-z,now -C link-arg=-Wl,-z,relro
RUSTFLAGS_DEV = -C debuginfo=2
.PHONY: all build dev $(TARGETS) dev-% docs
all: build dev
build: $(TARGETS)
dev: $(foreach t,$(TARGETS),dev-$(t))
# --- (Pattern Rules) ---
$(TARGETS): 
	@FLAGS="$(RUSTFLAGS_RELEASE)"; \
	PREFIX=""; \
	if echo "$@" | grep -q "musl"; then FLAGS="$$FLAGS -C link-arg=-lgcc_eh"; fi; \
	if echo "$@" | grep -q "none"; then FLAGS="$$FLAGS -C link-arg=-nostartfiles"; fi; \
	if echo "$@" | grep -q "aarch64"; then FLAGS="$$FLAGS -C linker=aarch64-linux-gnu-gcc"; PREFIX="aarch64-linux-gnu-"; fi; \
	if echo "$@" | grep -q "riscv64gc"; then FLAGS="$$FLAGS -C linker=riscv64-linux-gnu-gcc"; PREFIX="riscv64-linux-gnu-"; fi; \
	if [ -n "$$PREFIX" ]; then \
		T_ENV=$$(echo "$@" | tr '-' '_'); \
		export CC_$$T_ENV=$${PREFIX}gcc; \
		export AR_$$T_ENV=$${PREFIX}ar; \
	fi; \
	RUSTFLAGS="$$FLAGS" cargo build --release --target $@

# Dynamic Flags in dev Compilation
dev-%: 
	@FLAGS="$(RUSTFLAGS_DEV)"; \
	PREFIX=""; \
	if echo "$*" | grep -q "musl"; then FLAGS="$$FLAGS -C link-arg=-lgcc_eh"; fi; \
	if echo "$*" | grep -q "none"; then FLAGS="$$FLAGS -C link-arg=-nostartfiles"; fi; \
	if echo "$*" | grep -q "aarch64"; then FLAGS="$$FLAGS -C linker=aarch64-linux-gnu-gcc"; PREFIX="aarch64-linux-gnu-"; fi; \
	if echo "$*" | grep -q "riscv64gc"; then FLAGS="$$FLAGS -C linker=riscv64-linux-gnu-gcc"; PREFIX="riscv64-linux-gnu-"; fi; \
	if [ -n "$$PREFIX" ]; then \
		T_ENV=$$(echo "$*" | tr '-' '_'); \
		export CC_$$T_ENV=$${PREFIX}gcc; \
		export AR_$$T_ENV=$${PREFIX}ar; \
	fi; \
	RUSTFLAGS="$$FLAGS" cargo build --target $*
# Generate docs for project
docs:
	cargo doc --open
