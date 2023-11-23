run:
	cargo build --release
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000


file:
	rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os

qemu:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

clean:
	cargo clean
