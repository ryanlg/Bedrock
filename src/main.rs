// =========== Modules ============
mod config;
mod program;
mod builder;

// ============ Import ============
use std::io;
use std::env;
use std::process::{exit, Command};

use crate::builder::Builder;
use crate::config::BUILD_PROFILE;
use crate::program::*;

fn main() {

    // Parse command line argument, right now there's just the profile
    let mut profile = Profile::Debug;

    // Skip the first arg, just like C it's the command invoking
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--release" => profile = Profile::Release,
            _           => die(1, "Usage: redstone [--release]"),
        }
    }

    println!("Start building bootloader");

    // Get all the build programs associated with the profile and bootloader
    let build_programs =
        BUILD_PROFILE.get_build_programs(profile, Component::Bootloader);

    let version_arg = ["--version"];

    // Check if all external programs needed are installed
    println!("External programs needed: {}, {}, {}, {}",
             build_programs.cargo.command,
             build_programs.linker.command,
             build_programs.objcopy.command,
             build_programs.assembler.command);

    if check_install(build_programs.cargo.command, &version_arg).is_err() {
        die(1, "Cargo is not found");
    }

    if check_install(build_programs.linker.command, &version_arg).is_err() {
        die(1, "Linker is not found");
    }

    if check_install(build_programs.assembler.command, &version_arg).is_err() {
        die(1, "Assembler is not found");
    }

    if check_install(build_programs.objcopy.command, &version_arg).is_err() {
        die(1, "Objcopy is not found");
    }

    println!("All programs installed, proceeding...");

    // Now we build the bootloader
    if Builder::build(Component::Bootloader, profile).is_err() {
        die(2, "Failed to build bootloader");
    }
}

/* Check whether a single program is installed */
fn check_install(command: &str, args: &[&str]) -> io::Result<()> {
    // Using .output() to consume stdout from external programs
    Command::new(command).args(args).output()?;

    Ok(())
}

/* Just die */
fn die(code: i32, msg: &str) -> ! {
    println!("Error: {}", msg);
    exit(code);
}
