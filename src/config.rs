use crate::program::*;

#[cfg(target_os = "macos")]
pub static BUILD_PROFILE: BuildProfile = BuildProfile {
    release: ComponentPrograms {

        bootloader: BuildPrograms {
            cargo: Program {
                command: "cargo",
                args: &[
                    "xbuild",
                    "--release",
                    "--target",
                    "../support/triples/i686-none-redstone.json",
                ]
            },
            linker: Program {
                command: "ld.lld",
                args: &[
                    "--nmagic",
                    "--omagic",
                    "--gc-sections",
                    "-Bstatic",
                    "--whole-archive",
                ],
            },
            objcopy: Program {
                command: "x86_64-elf-objcopy",
                args: &[
                    "-O",
                    "binary",
                    "--only-section=.text",
                ]
            },
            assembler: Program {
                command: "nasm",
                args: &[
                    "-f",
                    "elf"
                ]
            }
        }

    },

    debug: ComponentPrograms {

        bootloader: BuildPrograms {
            cargo: Program {
                command: "cargo",
                args: &[
                    "xbuild",
                    "--target",
                    "../support/triples/i686-none-redstone.json",
                ]
            },
            linker: Program {
                command: "ld.lld",
                args: &[
                    "--nmagic",
                    "--omagic",
                    "--gc-sections",
                    "-Bstatic",
                    "--whole-archive",
                ],
            },
            objcopy: Program {
                command: "x86_64-elf-objcopy",
                args: &[
                    "-O",
                    "binary",
                ]
            },
            assembler: Program {
                command: "nasm",
                args: &[
                    "-f",
                    "elf"
                ]
            }
        }

    }
};

// ================================================
pub static BOOTLOADER_ARCH:   &str   = "x86";
pub static BOOTLOADER_LIB:    &str   = "libredstone_bootloader.a";
pub static BOOTLOADER_LINKED: &str   = "redstone-bootloader-x86.a";
pub static BOOTLOADER_FLAT:   &str   = "redstone-bootloader-x86.bin";
pub static BOOTLOADER_PADDED: &str   = "redstone-bootloader-x86.padded.bin";
pub static BOOTLOADER_LINKER_SCRIPT: &str = "linker.ld";
