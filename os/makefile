
all:
	cargo build
	cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/debug/os -O binary target/riscv64gc-unknown-none-elf/debug/os.bin
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin

run:
	cargo build
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/debug/os -O binary target/riscv64gc-unknown-none-elf/debug/os.bin
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000


file:
	rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os

qemu:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000

qemu-gdb:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/debug/os.bin,addr=0x80200000 \
		-s -S


run-release:
	cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000


file-release:
	rust-readobj -h target/riscv64gc-unknown-none-elf/release/os

qemu-release:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

qemu-gdb-release:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
		-s -S

clean:
	cargo clean
