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

use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::shells;
use std::env;
use std::io;

include!("../src/cli.rs");

fn main() -> Result<(), io::Error> {
    let out_dir = format!("{}/../../../", env::var("OUT_DIR").unwrap());
    let bin_name = env::var("CARGO_PKG_NAME").unwrap();

    let mut cmd = CLI::command();

    // Generate shell completion files
    generate_to(shells::Bash, &mut cmd, &bin_name, &out_dir)?;
    generate_to(shells::Zsh, &mut cmd, &bin_name, &out_dir)?;
    generate_to(shells::Fish, &mut cmd, &bin_name, &out_dir)?;

    Ok(())
}
