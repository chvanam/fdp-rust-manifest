//! # FDP Definition
//!
//! This crate contains the definitions of the apps within the FDP system, placed in the `apps` module.
//!
//! This crate also contains the following binaries:
//! - `doc`: Generates the documentation for the FDP system, to be viewed using `cargo doc --open`.
//! - `python`: Generates corresponding Python definitions for the FDP system.

pub mod apps;

/// UNUSED: Macro used to declare a Rust module inside 'apps' to be a FDP app definition
#[macro_export]
macro_rules! define_apps {
    ($($app:ident),*) => {
        $(
            // Needed for the Rust module structure
            pub mod $app;
        )*

        /// Returns the definition of the FDP system
        pub fn get_definition() -> fdp_common::info::SystemDefinitionInfo {
            fdp_common::info::SystemDefinitionInfo::from(Vec::from([
                $(
                    (
                        stringify!($app).to_string(),
                        $app::get_definition(),
                    ),
                )*
            ]))
        }
    };
}
