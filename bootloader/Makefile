arch                      := x86
triple                    := i686-none-redstone
exe_format                := elf

root_dir                  := ..
triple_dir                := $(root_dir)/support/triples
src_dir                   := src
src_arch_dir              := $(src_dir)/arch/$(arch)

build_dir                 := $(root_dir)/build/bootloader
build_arch_dir             = $(build_dir)/arch/$(arch)
build_name                := redstone-bootloader-$(arch)

build_linked               = $(build_dir)/$(build_name).a
build_stripped             = $(build_dir)/$(build_name).stripped.a
build_flattened            = $(build_dir)/$(build_name).bin

target_dir                := $(root_dir)/target

# ============ Source/Objects ==============
# Suffix
asm_suffix           := .s

# Objects
asm_src               = $(wildcard $(src_arch_dir)/*.s)
asm_objs              = $(addprefix $(build_arch_dir)/, $(notdir $(asm_src:.s=.s.o)))

rust_target_dir      := $(target_dir)

debug:   rust_lib_dir = $(rust_target_dir)/$(triple)/debug
release: rust_lib_dir = $(rust_target_dir)/$(triple)/release

debug:   rust_lib    = $(rust_lib_dir)/libredstone_bootloader.a
release: rust_lib    = $(rust_lib_dir)/libredstone_bootloader.a
rust_sysroot         := $(target_dir)/sysroot/$(triple)

# =================== Programs/Flags ==================
CC                   := clang
CPP                  := clang++
LINKER               := ld.lld-10
ASSEMBLER            := nasm
CARGO                := cargo
STRIPPER             := strip
OBJCOPY              := objcopy

# ld.lld
linker_args          := --nmagic --omagic --gc-sections -Bstatic --whole-archive -Bdynamic
linker_script        := $(src_arch_dir)/linker.ld

# Rust
cargo_cmd            := xbuild

cargo_arg_target     := --target $(root_dir)/support/triples/i686-none-redstone.json
debug  : cargo_arg   := $(cargo_arg_target)
release: cargo_arg   := $(cargo_arg_target) --release

# ==================== Targets ====================
.PHONY: all debug release
all: debug

debug:   $(build_flattened)
release: $(build_flattened)

# Assemble .s
$(build_arch_dir)/%.s.o: $(src_arch_dir)/%.s
	@$(ASSEMBLER) -f $(exe_format) $< -o $@

# Rust
.PHONY: rust_build
rust_build:
	@XBUILD_SYSROOT_PATH=$(rust_sysroot) $(CARGO) $(cargo_cmd) $(cargo_arg) --target-dir $(rust_target_dir)

# Linker
$(build_linked): rust_build $(asm_objs)
	@$(LINKER) $(linker_args) -o $(build_linked) -T $(linker_script) $(asm_objs) $(rust_lib)

# Strip the elf
$(build_stripped): $(build_linked)
	@$(STRIPPER) $(build_linked) -o $(build_stripped)

# Flatten to a binary
$(build_flattened): $(build_stripped)
	@$(OBJCOPY) -O binary $(build_stripped) $(build_flattened)
