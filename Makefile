# --- Config ---
# Linux x86_64 (Standard Intel/AMD Linux - static compilation)
# Linux ARM64 (e.g., Raspberry Pi 4/5, AWS Graviton servers)
# Linux RISC-V (64-bit RISC-V devices)
# Linux None (Your Own Kernel)
TARGETS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu riscv64gc-unknown-linux-gnu \
          x86_64-unknown-linux-musl aarch64-unknown-linux-musl riscv64gc-unknown-linux-musl
#		  x86_64-unknown-linux-none aarch64-unknown-linux-none riscv64gc-unknown-linux-none
RUSTFLAGS_RELEASE = -C link-arg=-Wl,-z,now -C link-arg=-Wl,-z,relro
RUSTFLAGS_DEV = -C debuginfo=2
.PHONY: all build dev $(TARGETS) run clean test check lint fmt docs verify
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

# Dynamiczne sterowanie flagami i linkerami dla kompilacji Dev
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
# Run Project
run:
	cargo run
# Clean deleting target folder
clean:
	cargo clean
# Testing
test:
	cargo test
# Check compilation errors
check:
	cargo check
# Linter - wyłapuje brzydki kod, błędy logiczne i anty-wzorce
lint:
	cargo clippy
# Formatowanie kodu (zgodnie ze standardem Rusta)
fmt:
	cargo fmt
# Generate docs for project
docs:
	cargo doc --open
# Sprawdzenie, czy wszystko jest git (szybki test przed commitem)
verify: fmt lint check test
