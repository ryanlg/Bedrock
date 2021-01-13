build_dir         := build/

bootloader_dir    := $(build_dir)/bootloader
bootloader_name   := redstone-bootloader-x86

bootloader_padded := $(bootloader_name).padded.bin

# =================== Programs ==================
CARGO    := cargo
QEMU     := qemu-system-x86_64
BOCHS    := bochs

qemu_drive_arg  := -drive format=raw,file=$(bootloader_dir)/$(bootloader_padded)
qemu_serial_arg := -serial stdio

bochs_drive_arg := 'ata0-master: type=disk, \
                                 path="build/bootloader/redstone-bootloader-x86.padded.bin", \
			 					 mode=flat, \
								 cylinders=1, \
								 heads=1, \
								 spt=1' \
					'boot: disk' \
					'megs: 128'

# =================== Targets ===================
.PHONY: debug release
all: debug

debug:
	@$(CARGO) run --release

release:
	@$(CARGO) run --release -- --release

# ---------------- Run ----------------
qemu:
	$(QEMU) $(qemu_drive_arg) $(qemu_serial_arg)

qemu-gdb:
	$(QEMU) $(qemu_drive_arg) $(qemu_serial_arg) -s -S

qemu-noui:
	$(QEMU) $(qemu_drive_arg) -nographic

qemu-noui-gdb:
	$(QEMU) $(qemu_drive_arg) -nographic -s -S

bochs:
	$(BOCHS) -qf /dev/null $(bochs_drive_arg)

