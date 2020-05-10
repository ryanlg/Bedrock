use std::io;
use std::process::Command;

/** Programs to be used when building */
pub struct Program {
    pub cargo:     &'static str,
    pub linker:    &'static str,
    pub objcopy:   &'static str,
    pub assembler: &'static str,
}

impl Program {

    /**
     * Check if all the programs are installed on the host system,
     * Here we assume if 
     */
    #[cfg(target_os = "macos")]
    pub fn check_install_all(&self) -> io::Result<()> {

        // Coincidentially all of our programs uses this as their version command
        let version_arg = ["--version"];

        self.check_install(self.cargo, &version_arg)?;
        self.check_install(self.linker, &version_arg)?;
        self.check_install(self.objcopy, &version_arg)?;
        self.check_install(self.assembler, &version_arg)?;

        Result::Ok(())
    }

    /** Check whether a single program is installed */
    fn check_install(&self, command: &str, args: &[&str]) -> io::Result<()> {

        // Using .output() to consume stdout from external programs
        Command::new(command).args(args).output()?;
        Result::Ok(())
    }
}

#[cfg(target_os = "macos")]
pub static PROGRAMS: Program = Program {
    cargo:     "cargo",
    linker:    "ld.lld",
    objcopy:   "x86_64elf-objcopy",
    assembler: "nasm",
};
