// =========== Modules ============
mod program;

// ============ Import ============
use program::PROGRAMS;
use std::process::exit;


fn main(){

    println!("Start building Redstone");

    // Check if all external programs needed are installed
    println!("External programs needed: {}, {}, {}, {}",
             PROGRAMS.cargo, PROGRAMS.linker, PROGRAMS.objcopy, PROGRAMS.assembler);
    match PROGRAMS.check_install_all() {
        Ok(_)  => println!("All programs installed, proceeding..."),
        Err(err) => die(1, "cannot find some external programs"),
    }
}

fn die(code: i32, msg: &str) -> ! {
    println!("Error: {}", msg);
    exit(code);
}
