BOOTLOADER = bootloader.asm
IMAGE = bootloader.img

all: $(IMAGE)

$(IMAGE): $(BOOTLOADER)
	nasm -f bin $(BOOTLOADER) -o $(IMAGE)

run: $(IMAGE)
	qemu-system-x86_64 -drive format=raw,file=$(IMAGE)

clean:
	rm -f $(IMAGE)