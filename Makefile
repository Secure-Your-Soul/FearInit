# Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)

# This file is part of FearInit and is dual-licensed:
#   1. AGPL-3.0-or-later — see LICENSE for the full text.
#   2. Commercial license — see COMMERCIAL.md for details.

# ==============================================================================
# FEARINIT - ARCHITECTURAL TARGETS & CPU VARIANTS DOCUMENTATION
# ==============================================================================

# 1. LINUX x86_64 (Standard Intel/AMD Linux - static compilation)
#    Branching into official microarchitecture levels (x86-64-levels):
#    - x86-64-v1: Base 64-bit from 2003 (Pentium 4, Athlon 64 - for absolute legacy hardware)
#    - x86-64-v2: Requires SSE4.2, POPCNT (CPUs from ~2009+, older virtual machines and VPS)
#    - x86-64-v3: Requires AVX2, BMI1, BMI2, FMA (Modern cloud servers AWS/GCP, the sweet spot)
#    - x86-64-v4: Requires AVX-512 (Latest, high-performance Intel/AMD server processors)

# 2. LINUX ARM64 (e.g., Raspberry Pi 4/5, AWS Graviton servers)
#    Branching into ARM core classes (AArch64):
#    - Power-efficient / IoT: Cortex-A34, Cortex-A35, Cortex-A53, Cortex-A55 (e.g., Raspberry Pi 3/4)
#    - Performance SBC / TV-Boxes: Cortex-A57, Cortex-A72 (e.g., Raspberry Pi 4 [A72]), Cortex-A73 to A76
#    - Modern flagships: Cortex-A510, Cortex-A710, Cortex-A715, Cortex-X1, Cortex-X2
#    - Cloud servers: Neoverse-N1, Neoverse-N2, Neoverse-V1 (e.g., AWS Graviton 2/3, Ampere Altra)
#    - Apple Silicon: apple-m1 (for Linux running via Asahi Linux / virtual machines)

# 3. LINUX RISC-V (64-bit RISC-V devices)
#    Branching into RISC-V core implementation standards:
#    - generic: Standard universal RV64GC profile (General + extensions A, C, D, F, M)
#    - SiFive Application Cores: sifive-u54, sifive-u74 (e.g., SiFive HiFive Unleashed / Unmatched boards)
#    - T-Head High-Performance: thead-c906 (e.g., Chinese SoCs in Allwinner D1 / Lichee Pi boards)

TARGETS = x86_64-unknown-linux-musl aarch64-unknown-linux-musl riscv64gc-unknown-linux-musl
#		  x86_64-unknown-none aarch64-unknown-none riscv64gc-unknown-none-elf

.PHONY: all build dev $(TARGETS) dev-% docker test test-qemu

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
	cd .tmp-initramfs && find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz
	@echo "⬇️  Downloading Alpine kernel..."
	# Download Alpine Kernel (vmlinuz-lts)
	curl -L -o vmlinuz-lts https://mirror.koddos.net/archlinux/iso/latest/arch/boot/x86_64/vmlinuz-linux
	@echo "🚀 Starting QEMU..."
	@rm -rf .tmp-initramfs
	qemu-system-x86_64 \
		-enable-kvm \
		-m 2048 \
		-cpu max \
		-nographic \
		-kernel vmlinuz-lts \
		-initrd initramfs.cpio.gz \
		-append "console=ttyS0"
	@rm initramfs.cpio.gz
	@rm vmlinuz-lts