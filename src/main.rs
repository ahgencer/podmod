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
use nix::unistd;
use podmod::config;
use std::env;
use std::path;

pub mod cli;

fn main() {
    // Ensure running on Linux
    if env::consts::OS != "linux" {
        panic!("Must run on Linux");
    }

    // Parse command line arguments and configuration file
    let args = cli::CLI::parse();
    let config: config::Config = config::parse(&args.config);

    let module_config = match args.command {
        cli::Command::Build { ref module, .. } |
        cli::Command::Load { ref module, .. } |
        cli::Command::Run { ref module, .. } |
        cli::Command::Shell { ref module, .. } |
        cli::Command::Unload { ref module, .. } => {
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
        cli::Command::Build { idempotent, no_prune, .. } => {
            podmod::build(&config, &module_config.unwrap(), idempotent, no_prune)
        },
        cli::Command::Load { idempotent, .. } => {
            podmod::load(&module_config.unwrap(), idempotent)
        },
        cli::Command::Modules {} => {
            podmod::modules(&config)
        },
        cli::Command::Run { command, .. } => {
            podmod::run(&module_config.unwrap(), &command);
        },
        cli::Command::Shell { shell, .. } => {
            podmod::shell(&module_config.unwrap(), &shell);
        }
        cli::Command::Unload { idempotent, .. } => {
            podmod::unload(&module_config.unwrap(), idempotent)
        }
    };
}
