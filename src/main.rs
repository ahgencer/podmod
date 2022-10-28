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
use podmod::*;
use std::collections;
use std::env;
use std::fs;
use toml;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[clap(short, long, default_value = "/etc/podmod.conf")]
    config: String,

    /// Use CONFIG as configuration
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Build the kernel module
    Build {
        /// Quietly exit if module is already built
        #[clap(short, long)]
        idempotent: bool,

        /// Work on the module MODULE
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

fn parse_config(path: &str) -> toml::Value {
    // Read file into String
    let file = fs::read_to_string(path)
        .expect(format!("Error while reading configuration file at {}", path).as_str());

    // Parse file using the 'toml' crate
    let config = file.parse::<toml::Value>()
        .expect(format!("Error while parsing configuration file at {}", path).as_str());

    config
}

fn get_main_config(config: &toml::Value) -> &str {
    // Fetch TOML values
    let data_dir = config.get("data_dir")
        .expect("Missing configuration option 'data_dir'")
        .as_str()
        .expect("Configuration option 'data_dir' must have a string value");

    data_dir
}

fn get_module_config<'a>(
    config: &'a toml::Value,
    module: &'a str,
) -> (
    &'a str,
    Vec<&'a str>,
    collections::HashMap<&'a str, &'a str>,
) {
    // Fetch parent TOML tables
    let config = config.get(&module)
        .expect(format!("Missing configuration for module {}", module).as_str())
        .as_table()
        .expect(format!("Configuration for module {} must be a table", module).as_str());

    let build_config = config.get("build")
        .expect(format!("Missing build configuration for module {}", module).as_str())
        .as_table()
        .expect(format!("Build configuration for module {} must be a table", module).as_str());

    // Fetch TOML values
    let version = config.get("version")
        .expect(format!("No version specified for module {}", module).as_str())
        .as_str()
        .expect(format!("Version identifier for module {} must have a string value", module).as_str());


    let kernel_args = config.get("kernel_args")
        .expect(format!("No kernel parameters specified for module {}", module).as_str())
        .as_array()
        .expect(format!("Kernel parameters for module {} must be an array", module).as_str());

    let kernel_args: Vec<_> = kernel_args.iter().map(|v| v.as_str().unwrap()).collect();

    let mut build_args = collections::HashMap::new();

    for (key, value) in build_config {
        let value = value.as_str()
            .expect(format!("Build parameter for module {} must have a string value", module).as_str());

        build_args.insert(key.as_str(), value);
    }

    (version, kernel_args, build_args)
}

fn main() {
    // Parse command line arguments and configuration file
    let args = Args::parse();
    let config = parse_config(&args.config);

    let data_dir = get_main_config(&config);

    // Ensure running on Linux
    if env::consts::OS != "linux" {
        panic!("Must run on Linux");
    }

    // Ensure program is run as root
    if !unistd::Uid::effective().is_root() {
        panic!("Must be run as root");
    }

    // Call appropriate functions from library
    match args.command {
        Command::Build { idempotent, module, no_prune } => {
            let (module_version, .., build_args) = get_module_config(&config, &module);
            build(data_dir, idempotent, &module, &module_version, no_prune, &build_args)
        }
        Command::Load { idempotent, module } => {
            let (module_version, kernel_args, ..) = get_module_config(&config, &module);
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
