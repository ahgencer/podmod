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

use std::collections;
use std::fs;
use std::path;
use std::process;
use std::str;

fn get_kernel_version() -> String {
    // Get kernel version
    let output = process::Command::new("uname")
        .arg("-r")
        .output()
        .expect("Error while fetching kernel version");

    // Parse stdout into str
    let output = str::from_utf8(&output.stdout[..]).unwrap();

    // Trim whitespace and newlines
    let output = str::trim(output);

    String::from(output)
}

fn get_image_identifier(module: &str, module_version: &str, kernel_version: &str) -> String {
    format!("{}-{}:{}-{}", env!("CARGO_PKG_NAME"), module, module_version, kernel_version)
}

fn module_is_loaded(module: &str) -> bool {
    let lsmod = process::Command::new("lsmod")
        .stdout(process::Stdio::piped())
        .spawn()
        .unwrap();

    let mut grep = process::Command::new("grep")
        .arg(&module)
        .stdin(process::Stdio::from(lsmod.stdout.unwrap()))
        .stdout(process::Stdio::null())
        .spawn()
        .unwrap();

    grep.wait().unwrap().success()
}

fn module_is_supported(data_dir: &str, module: &str) -> bool {
    let path = format!("{}/modules/{}", data_dir, module);
    path::Path::new(&path).is_dir()
}

fn image_exists(identifier: &str) -> bool {
    process::Command::new("podman")
        .args(["image", "exists", identifier])
        .status()
        .unwrap()
        .success()
}

pub fn build(
    data_dir: &str,
    idempotent: bool,
    kernel_version: Option<String>,
    module: &str,
    module_version: &str,
    no_prune: bool,
    build_args: &collections::HashMap<&str, &str>,
) {
    // Ensure module is supported
    if !module_is_supported(&data_dir, &module) {
        panic!("Module {} is not supported", module);
    }

    // Get kernel version
    let kernel_version = match kernel_version {
        Some(version) => version,
        None => get_kernel_version(),
    };

    // Get CPU architecture
    let arch = process::Command::new("uname")
        .arg("-p")
        .output()
        .expect("Error while fetching CPU architecture");

    let arch = str::from_utf8(&arch.stdout).unwrap();
    let arch = str::trim(arch);

    // Get image identifier
    let image_name = get_image_identifier(&module, &module_version, &kernel_version);

    // Check for existing image
    if image_exists(&image_name) {
        if idempotent {
            return;
        }

        panic!("Module {} is already built", module);
    }

    println!("Building module {} for kernel version {} ...", module, kernel_version);

    // Build new container image
    let mut command = process::Command::new("podman");
    command.args(["build", "-t", &image_name])
        .args(["--build-arg", format!("ARCH={}", arch).as_str()])
        .args(["--build-arg", format!("KERNEL_VERSION={}", kernel_version).as_str()])
        .args(["--build-arg", format!("MODULE_VERSION={}", module_version).as_str()]);

    for (key, value) in build_args {
        command.args(["--build-arg", format!("{}={}", key, value).as_str()]);
    }

    let success = command.arg(format!("{}/modules/{}", data_dir, module))
        .status()
        .unwrap()
        .success();

    assert!(success, "Error while running build kernel module");

    // Prune intermediary images
    if !no_prune {
        let success = process::Command::new("podman")
            .args(["system", "prune", "-f"])
            .status()
            .unwrap()
            .success();

        assert!(success, "Error while pruning old images");
    }
}

pub fn load(idempotent: bool, module: &str, module_version: &str, kernel_args: &Vec<&str>) {
    let kernel_version = get_kernel_version();
    let image_name = get_image_identifier(&module, &module_version, &kernel_version);

    // Ensure module is built
    if !image_exists(&image_name) {
        panic!("Module {} is not built", module);
    }

    // Ensure module is not loaded
    if module_is_loaded(&module) {
        if idempotent {
            return;
        }

        panic!("Module {} is already loaded", module);
    }

    println!("Loading module {} ...", module);

    // Run insmod inside new container
    let success = process::Command::new("podman")
        .args(["run", "--rm", "--privileged", &image_name])
        .args(["insmod", format!("/usr/lib/modules/{}/extra/{}.ko", kernel_version, module).as_str()])
        .args(kernel_args)
        .status()
        .unwrap()
        .success();

    assert!(success, "Error while loading kernel module");
}

pub fn modules(data_dir: &str) {
    println!("The following kernel modules are supported:");

    // Read file paths in modules data directory
    let modules = fs::read_dir(format!("{}/modules", data_dir))
        .expect("Error while reading data directory");

    for module in modules {
        // Get file name
        let module = module.unwrap().path();
        let module = module.file_name();

        // Get basename
        let module = module.unwrap().to_str().unwrap();

        println!("{}", module);
    }
}

pub fn unload(idempotent: bool, module: &str) {
    // Ensure module is loaded
    if !module_is_loaded(&module) {
        if idempotent {
            return;
        }

        panic!("Module {} is not loaded", module);
    }

    println!("Unloading module {} ...", module);

    // Run rmmod (doesn't need to be inside a container)
    let success = process::Command::new("rmmod")
        .arg(format!("{}", module))
        .status()
        .unwrap()
        .success();

    assert!(success, "Error while unloading kernel module");
}
