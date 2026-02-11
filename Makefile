SRC_DIR := src
BIN_DIR := bin

ASM_SRC := $(SRC_DIR)/boot.asm
RUST_SRC := $(SRC_DIR)/kernel.rs
LINKER_SCRIPT := $(SRC_DIR)/linker.ld

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

$(KERNEL_BIN): $(BOOT_OBJ) $(KERNEL_OBJ) $(LINKER_SCRIPT)
	$(LD) $(LDFLAGS) -o $@ $(BOOT_OBJ) $(KERNEL_OBJ)

.PHONY: run
run: $(KERNEL_BIN)
	qemu-system-i386 -kernel bin/myos.bin -m 512 -audiodev alsa,id=audio0 -machine pcspk-audiodev=audio0

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
	
	
# New Variables
ISO_DIR := iso_root
MYOS_ISO := $(BIN_DIR)/myos.iso

# ISO Generation Target
iso: $(KERNEL_BIN)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL_BIN) $(ISO_DIR)/boot/myos.bin
	cp grub.cfg $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(MYOS_ISO) $(ISO_DIR)
	@echo "ISO created at $(MYOS_ISO)"
