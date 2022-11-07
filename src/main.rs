/*
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/*
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use clap::Parser;
use clap::Subcommand;
use nix::unistd;
use podmod::config;
use std::env;
use std::path;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Path to the configuration file
    #[clap(short, long, default_value = "/etc/podmod.conf")]
    pub config: String,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Build the kernel module
    Build {
        /// Quietly exit if module is already built
        #[clap(short, long)]
        idempotent: bool,

        /// The module to work on
        #[clap(short, long)]
        module: String,

        /// Don't prune old images after building
        #[clap(long)]
        no_prune: bool,
    },

    /// Load the kernel module
    Load {
        /// Quietly exit if module is already loaded
        #[clap(short, long)]
        idempotent: bool,

        /// The module to work on
        #[clap(short, long)]
        module: String,
    },

    /// List supported kernel modules
    Modules {},

    /// Unload the kernel module
    Unload {
        /// Quietly exit if module is not loaded
        #[clap(short, long)]
        idempotent: bool,

        /// Work on the module MODULE
        #[clap(short, long)]
        module: String,
    },
}

fn main() {
    // Ensure running on Linux
    if env::consts::OS != "linux" {
        panic!("Must run on Linux");
    }

    // Parse command line arguments and configuration file
    let args = Args::parse();
    let config: config::Config = config::parse(&args.config);

    let module_config = match args.command {
        Command::Build { ref module, .. } |
        Command::Load { ref module, .. } |
        Command::Unload { ref module, .. } => {
            Some(config::module(&config.tree, &module))
        }
        _ => None,
    };

    // Ensure program is run as root
    if !unistd::Uid::effective().is_root() {
        panic!("Must be run as root");
    }

    // Ensure data directory is found
    if !path::Path::new(&config.data_dir).is_dir() {
        panic!("Data directory does not exist")
    }

    // Call appropriate function from library
    match args.command {
        Command::Build { idempotent, no_prune, .. } => {
            podmod::build(&config, &module_config.unwrap(), idempotent, no_prune)
        },
        Command::Load { idempotent, .. } => {
            podmod::load(&module_config.unwrap(), idempotent)
        },
        Command::Modules {} => {
            podmod::modules(&config)
        },
        Command::Unload { idempotent, .. } => {
            podmod::unload(&module_config.unwrap(), idempotent)
        }
    };
}
