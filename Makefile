arch                 := x86_64
target_triple        := x86_64-none-redstone
exe_arch             := elf64

srcdir               := src
src_archdir          := $(srcdir)/arch/$(arch)

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

# Rust
rust_src             := $(srcdir)
rust_target_debug    := target/$(target_triple)/debug
rust_target_release  := target/$(target_triple)/release

debug   :rust_kernel_lib := $(rust_target_debug)/libredstone.a
release :rust_kernel_lib := $(rust_target_release)/libredstone.a

# =========== Programs/Flags ===============
# Programs
VM           := qemu-system-x86_64
CC           := clang-10
CPP          := clang++-10
LINKER       := ld.lld-10
ASSEMBLER    := nasm
CARGO        := cargo

# Flags
VMFLAGS      := -cdrom $(distiso) -nographic -curses

# ================= Args ===================
# ld.lld
linker_args          := --nmagic --omagic --gc-sections -Bstatic --whole-archive -Bdynamic

# Rust
cargo_cmd            := xbuild

debug  : cargo_arg   :=
release: cargo_arg   := --release


# ================ Targets =================
# default target
.PHONY: all debug
all: debug

debug: $(distiso) run
release: $(distiso)

# Assemble .s
$(build_arch_dir)/%.s.o: $(src_archdir)/%.s
	@$(ASSEMBLER) -f $(exe_arch) $< -o $@

# Linker
$(distkernel): rust_build $(asm_objs) $(linker_script)
	@$(LINKER) $(linker_args) -T $(linker_script) -o $(distkernel) $(asm_objs) $(rust_kernel_lib)

# Make ISO
build_iso_dir  := $(build_arch_dir)/isofiles
$(distiso): $(distkernel) $(grub_config)
	@echo $(grub_config)
	@mkdir         -p $(build_iso_dir)/boot/grub
	@cp               $(distkernel)  $(build_iso_dir)/boot/$(build_kernel_name)
	@cp               $(grub_config) $(build_iso_dir)/boot/grub/$(notdir $(grub_config))
	@grub-mkrescue -o $(distiso)     $(build_iso_dir)
	@rm            -r $(build_iso_dir)

# ----------------- Rust ------------------
# Compile Rust src
# Always run, let the compiler deal with file changes
.PHONY: rust_build
rust_build:
	@$(CARGO) $(cargo_cmd) $(cargo_arg)


# ================== Run ==================
.PHONY: run
run:
	@$(VM) $(VMFLAGS)

# =============== Clean ====================
.PHONY: clean clean-iso clean-kernel clean-target clean-sysroot
clean: clean-build clean-dist clean-target
clean-dist: clean-iso clean-kernel
clean-iso:
	@rm -rf $(distiso)
clean-kernel:
	@rm -rf $(distkernel)
clean-build:
	@rm -rf $(build_arch_dir)/*
clean-target:
	@rm -rf target/release
	@rm -rf target/debug
	@rm -rf target/$(target_triple)
clean-sysroot:
	@rm -rf target/sysroot
