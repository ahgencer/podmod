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

use std::fs;
use std::process::Command;
use std::str;

pub fn build(data_dir: &String, module: String, kernel_version: Option<String>) {
    let kernel_version = match kernel_version {
        Some(version) => version,
        None => {
            let output = Command::new("uname").arg("-r").output().expect("Error while fetching kernel version");
            String::from(str::trim(str::from_utf8(&output.stdout[..]).unwrap()))
        }
    };

    println!("Building module {} for kernel version {}", module, kernel_version);
}

pub fn load(data_dir: &String, module: String) {
    println!("Loading module {}", module);
}

pub fn modules(data_dir: &String) {
    println!("The following kernel modules are supported:");

    let modules = fs::read_dir(format!("{}/modules", data_dir)).expect("Error while reading data directory");

    for module in modules {
        println!("{}", module.unwrap().path().file_name().unwrap().to_str().unwrap())
    }
}

pub fn unload(data_dir: &String, module: String) {
    println!("Unloading module {}", module);
}
