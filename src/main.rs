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

use clap::{Parser, Subcommand};
use nix::unistd::Uid;
use podmod::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use toml::Value;
use toml::value::Table;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[clap(short, long, default_value = "/etc/podmod.conf")]
    config: String,

    /// Use CONFIG as configuration
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build the kernel module
    Build {
        /// Quietly exit if module is already built
        #[clap(short, long)]
        idempotent: bool,

        /// Target the kernel version KERNEL_VERSION. [default: the current kernel version]
        #[clap(long)]
        kernel_version: Option<String>,

        /// Work on the module MODULE
        #[clap(short, long)]
        module: String,

        /// Don't prune old images after building
        #[clap(long)]
        no_prune: bool,
    },

    /// Load the kernel module. The module must already be built for this
    Load {
        /// Quietly exit if module is already loaded
        #[clap(short, long)]
        idempotent: bool,

        /// Work on the module MODULE
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

fn parse_config(path: &str) -> Value {
    // Read file into string
    let file = fs::read_to_string(path)
        .expect(format!("Error while reading configuration file at {}", path).as_str());

    // Parse string into TOML value
    let config = file.parse::<Value>()
        .expect(format!("Error while parsing configuration file at {}", path).as_str());

    config
}

fn main() {
    // Parse command line arguments and configuration file
    let args = Args::parse();
    let config = parse_config(&args.config);

    let data_dir = match config.get("data_dir") {
        Some(value) => value.as_str().expect("Configuration option 'data_dir' must have a string value"),
        None => "/usr/share/podmod",
    };

    // Ensure running on Linux
    if env::consts::OS != "linux" {
        panic!("Must run on Linux");
    }

    // Ensure program is run as root
    if !Uid::effective().is_root() {
        panic!("Must be run as root");
    }

    // Call appropriate functions from library
    match args.command {
        Command::Build { idempotent, kernel_version, module, no_prune } => {
            let default = Table::new();
            let module_config = match config.get(&module) {
                Some(value) => value.as_table().unwrap(),
                None => &default,
            };

            let module_version = match module_config.get("version") {
                Some(value) => value.as_str().unwrap(),
                None => panic!("Must specify module version for {}", module),
            };

            let default = Table::new();
            let module_build_config = match module_config.get("build") {
                Some(value) => value.as_table().unwrap(),
                None => &default,
            };

            let mut build_args = HashMap::new();
            for (key, value) in module_build_config {
                let value = value.as_str().unwrap();
                build_args.insert(key.as_str(), value);
            }

            build(data_dir, idempotent, kernel_version, &module, &module_version, no_prune, &build_args)
        }
        Command::Load { idempotent, module } => {
            let default = Table::new();
            let module_config = match config.get(&module) {
                Some(value) => value.as_table().unwrap(),
                None => &default,
            };

            let module_version = match module_config.get("version") {
                Some(value) => value.as_str().unwrap(),
                None => panic!("Must specify module version for {}", module),
            };

            let default = Vec::new();
            let kernel_args = match module_config.get("kernel_args") {
                Some(value) => value.as_array().expect("Configuration option 'kernel_args' must have an array value"),
                None => &default,
            };

            let kernel_args: Vec<_> = kernel_args.iter().map(|v| v.as_str().unwrap()).collect();

            load(idempotent, &module, &module_version, &kernel_args)
        }
        Command::Modules {} => {
            modules(data_dir)
        }
        Command::Unload { idempotent, module } => {
            unload(idempotent, &module)
        }
    };
}
