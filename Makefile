#directories
BOOT_SRC       := src/bootloader
KERNEL_SRC     := src/kernel

#output files
BOOTLOADER_BIN := build/bootloader.bin
KERNEL_BIN     := build/kernel.bin
FLOPPY_IMG     := build/main_floppy.img

#floppy image config
FLOPPY_LABEL   := NBOS
FLOPPY_SIZE    := 2880   
SECTOR_SIZE    := 512

.PHONY: all clean run bootloader kernel floppy_image always


#main target
all: $(FLOPPY_IMG)


# Special Makefile variables used:
# $@ = name of the target
# $< = the first pre-req

#floppy image
$(FLOPPY_IMG): bootloader kernel
	@echo "Creating floppy image..."
	dd if=/dev/zero of=$@ bs=$(SECTOR_SIZE) count=$(FLOPPY_SIZE)
	mkfs.fat -F 12 -n "$(FLOPPY_LABEL)" $@
	mcopy -i $@ $(KERNEL_BIN) ::kernel.bin
	dd if=$(BOOTLOADER_BIN) of=$@ conv=notrunc

bootloader: $(BOOTLOADER_BIN)

$(BOOTLOADER_BIN): $(BOOT_SRC)/boot.asm
	@mkdir -p build
	@echo "Assembling bootloader..."
	nasm $< -f bin -o $@

kernel: $(KERNEL_BIN)

$(KERNEL_BIN): $(KERNEL_SRC)/main.asm
	@mkdir -p build
	@echo "Assembling kernel..."
	nasm $< -f bin -o $@

always:
	@mkdir -p build

#run with QEMU
run: $(FLOPPY_IMG)
	qemu-system-i386 -fda $<

clean:
	@echo "Cleaning build directory..."
	rm -rf build/*