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

test-qemu:
	@echo "🔨 Building FearInit..."
	cargo build --release --target x86_64-unknown-linux-musl
	@echo "📦 Creating initramfs..."
	rm -rf .tmp-initramfs
	bash -c "mkdir -p .tmp-initramfs/{bin,dev,proc,sys}"
	cp target/x86_64-unknown-linux-musl/release/FearInit .tmp-initramfs/init
	chmod +x .tmp-initramfs/init
	mknod -m 600 .tmp-initramfs/dev/console c 5 1 || true
	mknod -m 666 .tmp-initramfs/dev/null c 1 3 || true
	cd .tmp-initramfs && find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz
	@echo "⬇️  Downloading Alpine kernel..."
	# Pobieramy gotowe jądro Alpine (vmlinuz-lts)
	curl -L -o vmlinuz-lts https://dl-cdn.alpinelinux.org/alpine/latest-stable/releases/x86_64/netboot/vmlinuz-lts
	@echo "🚀 Starting QEMU..."
	@rm -rf .tmp-initramfs
	qemu-system-x86_64 \
		-enable-kvm \
		-m 2048 \
		-nographic \
		-kernel vmlinuz-lts \
		-initrd initramfs.cpio.gz \
		-append "console=ttyS0"
	@rm initramfs.cpio.gz
	@rm vmlinuz-lts