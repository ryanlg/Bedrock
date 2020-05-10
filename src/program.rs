/** Programs to be used when building */
pub struct Program {
    pub cargo:     &'static str,
    pub linker:    &'static str,
    pub objcopy:   &'static str,
    pub assembler: &'static str,
}

#[cfg(target_os = "macos")]
pub static PROGRAMS: Program = Program {
    cargo:     "cargo",
    linker:    "ld.lld",
    objcopy:   "x86_64-elf-objcopy",
    assembler: "nasm",
};
