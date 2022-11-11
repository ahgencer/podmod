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

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct CLI {
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

    /// Run a command inside a new container
    Run {
        /// The module to work on
        #[clap(short, long)]
        module: String,

        /// The command to execute
        command: Vec<String>,
    },

    /// Start a shell session inside a new container
    Shell {
        /// The module to work on
        #[clap(short, long)]
        module: String,

        /// The shell command to run
        #[clap(default_value = "/bin/bash")]
        shell: String,
    },

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
