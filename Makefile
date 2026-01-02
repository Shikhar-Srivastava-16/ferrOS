# cargo clean; cargo build; cargo bootimage; qemu-system-x86_64 -debugcon stdio -drive format=raw,file=target/x86_64-ferrOS-targs/debug/bootimage-ferrOS.bin

all: newrun

clean:
	cargo clean

build:
	cargo build
	cargo bootimage

cleanbuild: clean build

run: build
	qemu-system-x86_64 \
		-debugcon stdio \
		-drive format=raw,file=target/x86_64-ferrOS-targs/debug/bootimage-ferrOS.bin

newrun: cleanbuild
	qemu-system-x86_64 \
		-debugcon stdio \
		-drive format=raw,file=target/x86_64-ferrOS-targs/debug/bootimage-ferrOS.bin
