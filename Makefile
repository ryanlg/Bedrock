arch                 := x86_64
exe_arch             := elf64

srcdir               := src/kernel
src_archdir              := $(srcdir)/arch/$(arch)

builddir             := build
build_arch_dir       := $(builddir)/arch/$(arch)
distdir              := dist

build_iso_name       := redstone-$(arch).iso
build_kernel_name    := redstone-kernel-$(arch).bin
distiso              := $(distdir)/$(build_iso_name)
distkernel           := $(distdir)/$(build_kernel_name)

# ============ Source/Objects ==============

# Suffix
asm_suffix           := .s

# Objects
asm_src              := $(wildcard $(src_archdir)/*.s)
asm_objs             := $(addprefix $(build_arch_dir)/, $(notdir $(asm_src:.s=.s.o)))

grub_config          := $(src_archdir)/grub.cfg
linker_script        := $(src_archdir)/linker.ld
linker_args          := --nmagic --omagic --gc-sections -Bstatic --whole-archive -Bdynamic

# =========== Programs/Flags ===============
# Programs
VM           := qemu-system-x86_64
CC           := clang
CPP          := clang++
LINKER       := ld.lld
ASSEMBLER    := nasm

# Flags
VMFLAGS      := -cdrom $(distiso)

# ================ Targets =================
# default target
.PHONY: all
all: $(distiso) run

# Assemble .s
$(build_arch_dir)/%.s.o: $(src_archdir)/%.s
	@$(ASSEMBLER) -f $(exe_arch) $< -o $@

# Linker
$(distkernel): $(asm_objs) $(linker_script)
	@$(LINKER) $(linker_args) -T $(linker_script) -o $(distkernel) $(asm_objs)

# Make ISO
build_iso_dir  := $(build_arch_dir)/isofiles
$(distiso): $(distkernel) $(grub_config)
	@echo $(grub_config)
	@mkdir         -p $(build_iso_dir)/boot/grub
	@cp               $(distkernel)  $(build_iso_dir)/boot/$(build_kernel_name)
	@cp               $(grub_config) $(build_iso_dir)/boot/grub/$(notdir $(grub_config))
	@grub-mkrescue -o $(distiso)     $(build_iso_dir) 2>/dev/null
	@rm            -r $(build_iso_dir)


# ================== Run ==================
.PHONY: run
run:
	@$(VM) $(VMFLAGS)

# =============== Clean ====================
.PHONY: clean clean-iso clean-kernel
clean: clean-build clean-dist
clean-dist: clean-iso clean-kernel
clean-iso:
	@rm -rf $(distiso)
clean-kernel:
	@rm -rf $(distkernel)
clean-build:
	@rm -rf $(build_arch_dir)/*
