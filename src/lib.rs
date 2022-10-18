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

fn get_kernel_version() -> String {
    let output = Command::new("uname").arg("-r").output().expect("Error while fetching kernel version");
    String::from(str::trim(str::from_utf8(&output.stdout[..]).unwrap()))
}

fn image_name(module: &String) -> String {
    format!("{}-{}", env!("CARGO_PKG_NAME"), module)
}

fn image_tag(kernel_version: &String) -> String {
    format!("{}", kernel_version)
}

pub fn build(data_dir: &String, module: String, kernel_version: Option<String>) {
    // Get kernel version
    let kernel_version = match kernel_version {
        Some(version) => version,
        None => get_kernel_version(),
    };

    // Get CPU architecture
    let arch = Command::new("uname")
        .arg("-p")
        .output()
        .expect("Error while fetching CPU architecture");

    let arch = String::from(str::trim(str::from_utf8(&arch.stdout[..]).unwrap()));

    println!("Building module {} for kernel version {}", module, kernel_version);

    // Build new container image
    Command::new("podman")
        .args(["build", "-t", format!("{}-{}", image_name(&module), image_tag(&kernel_version))])
        .args(["--build-arg", format!("ARCH={}", arch)])
        .args(["--build-arg", format!("KERNEL_VERSION={}", kernel_version)])
        .arg(format!("{}/modules/{}", data_dir, module))
        .status()
        .expect("Error while running build kernel module");
}

pub fn load(module: String) {
    let kernel_version = get_kernel_version();

    println!("Loading module {}", module);

    // Run insmod inside new container
    Command::new("podman")
        .args(["run", "--rm", "--privileged"])
        .arg(format!("{}-{}", image_name(&module), image_tag(&kernel_version)))
        .args(["insmod", format!("/usr/lib/modules/{}/extra/{}.ko", kernel_version, module)])
        .status()
        .expect("Error while loading kernel module");
}

pub fn modules(data_dir: &String) {
    println!("The following kernel modules are supported:");

    // Read file paths in modules data directory
    let modules = fs::read_dir(format!("{}/modules", data_dir))
        .expect("Error while reading data directory");

    for module in modules {
        // Get file name
        let module = module.unwrap().path().file_name();

        // Get basename
        let module = module.unwrap().to_str().unwrap();

        println!(module);
    }
}

pub fn unload(module: String) {
    println!("Unloading module {}", module);

    // Run rmmod (doesn't need to be inside a container)
    Command::new("rmmod")
        .arg(format!("{}", module))
        .status()
        .expect("Error while unloading kernel module");
}
