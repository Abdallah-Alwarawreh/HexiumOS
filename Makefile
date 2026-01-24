
SRC_DIR := src
BIN_DIR := bin

ASM_SRC := $(SRC_DIR)/boot.asm
RUST_SRC := $(SRC_DIR)/kernel.rs
LINKER_SCRIPT := $(SRC_DIR)/linker.ld
SHIM_SRC := $(SRC_DIR)/shim.c
SHIM_OBJ := $(BIN_DIR)/shim.o

BOOT_OBJ := $(BIN_DIR)/boot.o
KERNEL_OBJ := $(BIN_DIR)/kernel.o
KERNEL_BIN := $(BIN_DIR)/myos.bin

AS := as
LD := ld
RUSTC := rustc

ASFLAGS := --32
LDFLAGS := -m elf_i386 -T $(LINKER_SCRIPT)
RUSTFLAGS := --target i686-unknown-linux-gnu --crate-type staticlib \
             -C opt-level=2 -C panic=abort -C relocation-model=static \
             -C target-feature=-sse,-sse2,+soft-float

.PHONY: all
all: $(KERNEL_BIN)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(BOOT_OBJ): $(ASM_SRC) | $(BIN_DIR)
	$(AS) $(ASFLAGS) $< -o $@

$(KERNEL_OBJ): $(RUST_SRC) | $(BIN_DIR)
	$(RUSTC) $(RUSTFLAGS) -o $@ $<


$(SHIM_OBJ): $(SHIM_SRC) | $(BIN_DIR)
	gcc -m32 -ffreestanding -c $< -o $@

$(KERNEL_BIN): $(BOOT_OBJ) $(KERNEL_OBJ) $(SHIM_OBJ) $(LINKER_SCRIPT)
	$(LD) $(LDFLAGS) -o $@ $(BOOT_OBJ) $(KERNEL_OBJ) $(SHIM_OBJ)

.PHONY: run
run: $(KERNEL_BIN)
	qemu-system-i386 -kernel $(KERNEL_BIN)

.PHONY: clean
clean:
	rm -rf $(BIN_DIR)

.PHONY: rebuild
rebuild: clean all

.PHONY: help
help:
	@echo "RustOS Build System"
	@echo "make        - Build the kernel"
	@echo "make run    - Build and run in QEMU"
	@echo "make clean  - Remove build artifacts"
	@echo "make rebuild - Clean and rebuild"
