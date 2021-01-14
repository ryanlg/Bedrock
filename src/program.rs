/* Act as an accessor for all the tools, components and profiles */
#[derive(Copy, Clone, Debug)]
pub enum Profile {
    Debug,
    Release,
}

#[derive(Copy, Clone, Debug)]
pub enum Component {
    Bootloader,
}

/* Represents one program and its args */
#[derive(Debug)]
pub struct Program {
    pub command:   &'static str,
    pub args:      &'static[&'static str],
}

/* Programs and args to be used when building */
#[derive(Debug)]
pub struct BuildPrograms {
    pub cargo:     Program,
    pub linker:    Program,
    pub objcopy:   Program,
    pub assembler: Program,
}

/* Programs to be used for each component */
#[derive(Debug)]
pub struct ComponentPrograms {
    pub bootloader: BuildPrograms,
}

/* Programs to be used for each build profile */
#[derive(Debug)]
pub struct BuildProfile {
    pub release: ComponentPrograms,
    pub debug:   ComponentPrograms,
}

impl BuildProfile {

    /* Get BuildPrograms associated with a profile and component */
    pub fn get_build_programs(&self, profile: Profile,
                              component: Component) -> &BuildPrograms {

        let current_profile = match profile {
            Profile::Release => &self.release,
            Profile::Debug   => &self.debug,
        };

        match component {
            Component::Bootloader => &current_profile.bootloader,
        }
    }
}
