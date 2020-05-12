use std::vec::Vec;
use std::path::Path;
use std::result::Result;
use std::process::{Command, Stdio, ExitStatus};

use crate::program::{Profile, Component, BuildPrograms};
use crate::config::*;

pub struct Builder;

impl Builder {

    pub fn build(component: Component, profile: Profile) -> Result<(), Error> {

        let build_programs =
            BUILD_PROFILE.get_build_programs(profile, component);

        match component {
            Component::Bootloader => Builder::build_bootloader(build_programs)?,
        }

        Ok(())
    }

    fn build_bootloader(programs: &BuildPrograms) -> Result<(), Error> {

        let base_path            = Path::new("bootloader");
        let src_arch_path        = Path::new("src/arch")
                                        .join(BOOTLOADER_ARCH)
                                        .as_path()
                                        .to_owned();

        let build_path           = Path::new("../build/bootloader");
        let build_rust_path      = Path::new("../build/bootloader/rust");
        let build_sysroot_path   = Path::new("../build/sysroot");

        // Save old pwd and switch to the bootloader's folder 
        let old_current_dir = std::env::current_dir()?;
        std::env::set_current_dir(&base_path)?;

        // Make the necessary folders first
        std::fs::create_dir_all(build_path)?;

        // Now we build stuff
        // We need all the .s file to pass into the assembler
        let asm_src = std::fs::read_dir(&src_arch_path)?
                         .filter_map(|result_entry| result_entry.ok())
                         .filter_map(|entry|        entry.path()
                                                         .into_os_string()
                                                         .into_string()
                                                         .ok())
                         .filter(|file_name|        file_name.ends_with(".s"));


        // Collect all the .s.o path, need them in linking
        let mut asm_objs = Vec::with_capacity(5);
        for src in asm_src {
            let output_file = src.replace(".s", ".s.o")
                                 .replace(src_arch_path.to_str().unwrap(),
                                             build_path.to_str().unwrap());
            let status = Command::new(programs.assembler.command)
                                 .args(programs.assembler.args)
                                 .arg(&src)
                                 .arg("-o")
                                 .arg(&output_file)
                                 .stdout(Stdio::inherit())
                                 .stderr(Stdio::inherit())
                                 .spawn()?
                                 .wait();
            asm_objs.push(output_file);
            Self::check_status(status)?;
        }

        // Invoke Cargo and build rust
        let status = Command::new(programs.cargo.command)
                             .env("XBUILD_SYSROOT_PATH", &build_sysroot_path)
                             .args(programs.cargo.args)
                             .arg("--target-dir")
                             .arg(&build_rust_path)
                             .arg("--out-dir") // @unstable: this is not stable
                             .arg(&build_path)
                             .arg("-Z")
                             .arg("unstable-options")
                             .stdout(Stdio::inherit())
                             .stderr(Stdio::inherit())
                             .spawn()?
                             .wait();
        Self::check_status(status)?;

        // Invoke linker
        let linker_script = src_arch_path.join(BOOTLOADER_LINKER_SCRIPT);
        let rust_lib      = build_path.join(BOOTLOADER_LIB);
        let build_linked  = build_path.join(BOOTLOADER_LINKED);
        let status = Command::new(programs.linker.command)
                             .args(programs.linker.args)
                             .arg("-T")
                             .arg(linker_script)
                             .arg("-o")
                             .arg(&build_linked)
                             .args(asm_objs)
                             .arg(rust_lib)
                             .stdout(Stdio::inherit())
                             .stderr(Stdio::inherit())
                             .spawn()?
                             .wait();
        Self::check_status(status)?;

        // Flatten the image to a binary
        let build_flat = build_path.join(BOOTLOADER_FLAT);
        let status = Command::new(programs.objcopy.command)
                             .args(programs.objcopy.args)
                             .arg(&build_linked)
                             .arg(&build_flat)
                             .stdout(Stdio::inherit())
                             .stderr(Stdio::inherit())
                             .spawn()?
                             .wait();
        Self::check_status(status)?;

        // Now we pad the binary to the nearest multiples of 512 bytes
        // First we copy the file
        let build_padded = build_path.join(BOOTLOADER_PADDED);
        std::fs::copy(&build_flat, &build_padded)?;

        // Now we pad
        const BLOCK_SIZE: u64 = 512;
        let flat_file  = std::fs::OpenOptions::new()
                                              .append(true)
                                              .open(&build_padded)?;
        let file_size  = flat_file.metadata()?
                                  .len();
        let padding    = file_size % BLOCK_SIZE;
        let padding    = if padding > 0 { BLOCK_SIZE - padding } else { 0 };
        flat_file.set_len(file_size + padding)?;

        std::env::set_current_dir(old_current_dir)?;

        Ok(())
    }

    fn check_status(status: std::io::Result<ExitStatus>) -> Result<(), Error> {
        if status?.success() {
            Ok(())
        } else {
            Err(Error {})
        }
    }
}

/* Rudimentary error type */
#[derive(Debug)]
pub struct Error;
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something bad happened while building")
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        println!("{}", error);
        Error {}
    }
}
