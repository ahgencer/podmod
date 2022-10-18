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
use std::env;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Path to shared architecture-independent files
    #[clap(long, default_value = "/usr/share/podmod")]
    data_dir: String,

    /// Use CONFIG as configuration
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build the kernel module
    Build {
        /// Target the kernel version KERNEL_VERSION. [default: the current kernel version]
        #[clap(long)]
        kernel_version: Option<String>,

        /// Work on the module MODULE. Required
        #[clap(short, long)]
        module: String,
    },

    /// Load the kernel module. The module must already be built for this
    Load {
        /// Work on the module MODULE. Required
        #[clap(short, long)]
        module: String,
    },

    /// List supported kernel modules
    Modules {},

    /// Unload the kernel module
    Unload {
        /// Work on the module MODULE. Required
        #[clap(short, long)]
        module: String,
    },
}

fn main() {
    // Let clap parse command line arguments
    let args = Args::parse();

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
        Command::Build { module, kernel_version } => build(&args.data_dir, &module, kernel_version),
        Command::Load { module } => load(&module),
        Command::Modules {} => modules(&args.data_dir),
        Command::Unload { module } => unload(&module),
    };
}
