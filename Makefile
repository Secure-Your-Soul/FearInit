# Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)

# This file is part of FearInit and is dual-licensed:
#   1. AGPL-3.0-or-later — see LICENSE for the full text.
#   2. Commercial license — see COMMERCIAL.md for details.

# --- Config ---
# Linux x86_64 (Standard Intel/AMD Linux - static compilation)
# Linux ARM64 (e.g., Raspberry Pi 4/5, AWS Graviton servers)
# Linux RISC-V (64-bit RISC-V devices)
# Linux None (Your Own Kernel)

TARGETS = x86_64-unknown-linux-musl aarch64-unknown-linux-musl riscv64gc-unknown-linux-musl
#		  x86_64-unknown-none aarch64-unknown-none riscv64gc-unknown-none-elf

RUSTFLAGS_RELEASE = -C link-arg=-Wl,-z,now -C link-arg=-Wl,-z,relro
RUSTFLAGS_DEV = -C debuginfo=2

.PHONY: all build dev $(TARGETS) dev-% docs

all: build dev-%

build: $(TARGETS)

# Target (release)
$(TARGETS):
	cargo build --release --target $@

# Target (dev)
dev-%:
	cargo build --target $*
	
docker:
	docker compose down --remove-orphans
	docker compose up --build --abort-on-container-exit --exit-code-from builder

test:
	docker compose run --rm test
