TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

CC = riscv64-unknown-elf-gcc

OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

C_SRC=$(shell find $(APP_DIR)/ -name '*.c')  
C_LIB=./src/ulibc.c
C_OBJ=$(C_SRC:%.c=%.o)          
C_LIBOBJ=$(C_SRC:%.c=%.o)          

CFLAGS = -nostdlib -ffreestanding -fno-common -mno-relax -mcmodel=medany -MD
CLDFLAGS = -Tsrc/linker.ld -z max-page-size=4096


$(C_OBJ):$(C_SRC)
	$(CC) -c $(CFLAGS) -o $@ $<

elf:
	@cargo build --release
	@echo $(APPS)
	@echo $(ELFS)
	@echo $(BINS)

binary: elf
	$(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)

build: binary
