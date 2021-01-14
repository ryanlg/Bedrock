use crate::program::*;

#[cfg(target_os = "macos")]
pub static BUILD_PROFILE: BuildProfile = BuildProfile {
    release: ComponentPrograms {

        bootloader: BuildPrograms {
            cargo: Program {
                command: "cargo",
                args: &[
                    "build",
                    "--release",
                    "--target",
                    "i686-unknown-linux-gnu",
                    "-Z",
                    "build-std=core,compiler_builtins",
                ]
            },
            linker: Program {
                command: "ld.lld",
                args: &[
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

    },

    debug: ComponentPrograms {

        bootloader: BuildPrograms {
            cargo: Program {
                command: "cargo",
                args: &[
                    "build",
                    "--target",
                    "i686-unknown-linux-gnu",
                    "-Z",
                    "build-std=core,compiler_builtins",
                    "--verbose",
                ]
            },
            linker: Program {
                command: "ld.lld",
                args: &[
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
pub static BOOTLOADER_LIB:    &str   = "libbedrock_bootloader.a";
pub static BOOTLOADER_LINKED: &str   = "bedrock-bootloader-x86.a";
pub static BOOTLOADER_FLAT:   &str   = "bedrock-bootloader-x86.bin";
pub static BOOTLOADER_PADDED: &str   = "bedrock-bootloader-x86.padded.bin";
pub static BOOTLOADER_LINKER_SCRIPT: &str = "linker.ld";
