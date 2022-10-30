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

fn get_architecture() -> String {
    // Call 'uname -p' to fetch the architecture
    let arch = process::Command::new("uname")
        .arg("-p")
        .output()
        .expect("Error while fetching CPU architecture");

    // Cleanup and parse stdout into str
    let arch = str::from_utf8(&arch.stdout).unwrap();
    let arch = str::trim(arch);

    // We need to return a String since we don't
    // know the size at compile-time
    String::from(arch)
}

fn get_kernel_version() -> String {
    // Call 'uname -r' to fetch the kernel release
    let output = process::Command::new("uname")
        .arg("-r")
        .output()
        .expect("Error while fetching kernel version");

    // Cleanup and parse stdout into str
    let output = str::from_utf8(&output.stdout).unwrap();
    let output = str::trim(output);

    // We need to return a String since we don't
    // know the size at compile-time
    String::from(output)
}

fn is_module_loaded(module: &str) -> bool {
    // Call 'lsmod | grep $module' to check for loaded module
    let lsmod = process::Command::new("lsmod")
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Error while fetching loaded kernel modules");

    let mut grep = process::Command::new("grep")
        .arg(&module)
        .stdin(process::Stdio::from(lsmod.stdout.unwrap()))
        .stdout(process::Stdio::null())
        .spawn()
        .expect("Error while searching for loaded kernel module");

    // 'grep' succeeds only if string is found
    grep.wait().unwrap().success()
}

fn is_module_supported(data_dir: &str, module: &str) -> bool {
    // If the module is supported, it must have a subdirectory under 'data_dir'
    let path = format!("{}/modules/{}", data_dir, module);
    path::Path::new(&path).is_dir()
}

fn get_build_image_identifier(kernel_version: &str) -> String {
    format!("{}-builder:{}", env!("CARGO_PKG_NAME"), kernel_version)
}

fn get_runtime_image_identifier(kernel_version: &str) -> String {
    format!("{}-runtime:{}", env!("CARGO_PKG_NAME"), kernel_version)
}

fn get_module_image_identifier(module: &str, module_version: &str, kernel_version: &str) -> String {
    format!("{}-{}:{}-{}", env!("CARGO_PKG_NAME"), module, module_version, kernel_version)
}

fn image_exists(identifier: &str) -> bool {
    // Call 'podman exists' to check for existing image
    // The command only succeeds if image is found
    process::Command::new("podman")
        .args(["image", "exists", identifier])
        .status()
        .expect("Error while checking for pre-existing image")
        .success()
}

pub fn build(
    data_dir: &str,
    idempotent: bool,
    module: &str,
    module_version: &str,
    no_prune: bool,
    build_args: &collections::HashMap<&str, &str>,
) {
    // Ensure module is supported
    if !is_module_supported(&data_dir, &module) {
        panic!("Module {} is not supported", module);
    }

    // We'll need some information about the system when
    // compiling the kernel module
    let kernel_version = get_kernel_version();
    let arch = get_architecture();

    let build_image_name = get_build_image_identifier(&kernel_version);
    let runtime_image_name = get_runtime_image_identifier(&kernel_version);
    let module_image_name = get_module_image_identifier(&module, &module_version, &kernel_version);

    // Check for existing image
    if image_exists(&module_image_name) {
        if idempotent {
            return;
        }

        panic!("Module {} is already built", module);
    }

    // Check for existing builder image
    if !image_exists(&build_image_name) {
        println!("Building builder image for kernel version {} ...", kernel_version);

        process::Command::new("podman")
            .args(["build", "-t", &build_image_name])
            .args(["--build-arg", format!("ARCH={}", arch).as_str()])
            .args(["--build-arg", format!("KERNEL_VERSION={}", kernel_version).as_str()])
            .args(["--file", "Builder.containerfile"])
            .arg(format!("{}/common/", data_dir))
            .status()
            .expect("Error while building the builder image");
    }

    // Check for existing runtime image
    if !image_exists(&runtime_image_name) {
        println!("Building runtime image for kernel version {} ...", kernel_version);

        process::Command::new("podman")
            .args(["build", "-t", &runtime_image_name])
            .args(["--build-arg", format!("KERNEL_VERSION={}", kernel_version).as_str()])
            .args(["--file", "Runtime.containerfile"])
            .arg(format!("{}/common/", data_dir))
            .status()
            .expect("Error while building the runtime image");
    }

    println!("Building module {} for kernel version {} ...", module, kernel_version);

    // Call 'podman build' to build the new image
    // We already now the target architecture and kernel version
    let mut command = process::Command::new("podman");

    command
        .args(["build", "-t", &module_image_name])
        .args(["--build-arg", format!("ARCH={}", arch).as_str()])
        .args(["--build-arg", format!("KERNEL_VERSION={}", kernel_version).as_str()])
        .args(["--build-arg", format!("MODULE_VERSION={}", module_version).as_str()]);

    // Add additional build parameter passed to the function
    for (key, value) in build_args {
        command.args(["--build-arg", format!("{}={}", key, value).as_str()]);
    }

    command
        .arg(format!("{}/modules/{}", data_dir, module))
        .status()
        .expect("Error while building the kernel module");

    // By default, we'll prune any intermediary images that the build generates
    // The user probably isn't building the same image multiple times,
    // so keeping the cached build stages isn't very useful
    if !no_prune {
        process::Command::new("podman")
            .args(["system", "prune", "-f"])
            .status()
            .expect("Error while pruning intermediary images");
    }
}

pub fn load(idempotent: bool, module: &str, module_version: &str, kernel_args: &Vec<&str>) {
    // podmod's container images are always named predictably
    let kernel_version = get_kernel_version();
    let image_name = get_module_image_identifier(&module, &module_version, &kernel_version);

    // Ensure module is built
    if !image_exists(&image_name) {
        panic!("Module {} is not built", module);
    }

    // Check if module is already loaded
    if is_module_loaded(&module) {
        if idempotent {
            return;
        }

        panic!("Module {} is already loaded", module);
    }

    println!("Loading module {} ...", module);

    // Call the load script inside a new container
    // Add additional kernel parameters passed to the function
    process::Command::new("podman")
        .args(["run", "--rm", "--privileged", &image_name, "load"])
        .args(kernel_args)
        .status()
        .expect("Error while loading the kernel module");
}

pub fn modules(data_dir: &str) {
    println!("The following kernel modules are supported:");

    // Each supported module has a subdirectory in 'data_dir'
    let modules = fs::read_dir(format!("{}/modules", data_dir))
        .expect("Error while reading data directory");

    for module in modules {
        // Print the path's basename
        let module = module.unwrap().path();
        let module = module.file_name();
        let module = module.unwrap().to_str().unwrap();
        println!("{}", module);
    }
}

pub fn unload(idempotent: bool, module: &str, module_version: &str) {
    // Check if module is loaded
    if !is_module_loaded(&module) {
        if idempotent {
            return;
        }

        panic!("Module {} is not loaded", module);
    }

    // podmod's container images are always named predictably
    let kernel_version = get_kernel_version();
    let image_name = get_module_image_identifier(&module, &module_version, &kernel_version);

    println!("Unloading module {} ...", module);

    // Call the unload script inside a new container
    process::Command::new("podman")
        .args(["run", "--rm", "--privileged", &image_name, "unload"])
        .status()
        .expect("Error while unloading the kernel module");
}
