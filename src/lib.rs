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
use std::path;
use std::process;
use std::str;

pub mod config;
mod fetch;

fn is_module_supported(data_dir: &str, module: &str) -> bool {
    // If the module is supported, it must have a subdirectory under 'data_dir'
    let path = format!("{}/modules/{}", data_dir, module);
    path::Path::new(&path).is_dir()
}

fn get_build_image_identifier(kernel_version: &str) -> String {
    format!("{}-builder:{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), kernel_version)
}

fn get_runtime_image_identifier(kernel_version: &str) -> String {
    format!("{}-runtime:{}-{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), kernel_version)
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

pub fn build(config: &config::Config, module: &config::ModuleConfig, idempotent: bool, no_prune: bool) {
    // Ensure module is supported
    if !is_module_supported(&config.data_dir, &module.name) {
        panic!("Module {} is not supported", module.name);
    }

    // We'll need some information about the system when
    // compiling the kernel module
    let kernel_version = fetch::kernel_version();
    let arch = fetch::architecture();
    let podmod_version = env!("CARGO_PKG_VERSION");

    let build_image_name = get_build_image_identifier(&kernel_version);
    let runtime_image_name = get_runtime_image_identifier(&kernel_version);
    let module_image_name = get_module_image_identifier(&module.name, &module.version, &kernel_version);

    // Check for existing image
    if image_exists(&module_image_name) {
        if idempotent {
            return;
        }

        panic!("Module {} is already built", module.name);
    }

    // Build builder image
    if !image_exists(&build_image_name) {
        println!("Building builder image for kernel version {} ...", kernel_version);

        process::Command::new("podman")
            .args(["build", "-t", &build_image_name])
            .args(["--build-arg", &format!("ARCH={}", arch)])
            .args(["--build-arg", &format!("KERNEL_VERSION={}", kernel_version)])
            .args(["--file", "Builder.containerfile"])
            .arg(format!("{}/common/", config.data_dir))
            .status()
            .expect("Error while building the builder image");
    }

    // Build runtime image
    if !image_exists(&runtime_image_name) {
        println!("Building runtime image for kernel version {} ...", kernel_version);

        process::Command::new("podman")
            .args(["build", "-t", &runtime_image_name])
            .args(["--build-arg", &format!("KERNEL_VERSION={}", kernel_version)])
            .args(["--build-arg", &format!("PODMOD_VERSION={}", podmod_version)])
            .args(["--file", "Runtime.containerfile"])
            .arg(format!("{}/common/", config.data_dir))
            .status()
            .expect("Error while building the runtime image");
    }

    println!("Building module {} for kernel version {} ...", module.name, kernel_version);

    // Build the new image
    // We already know the target architecture and kernel version
    let mut command = process::Command::new("podman");

    command
        .args(["build", "-t", &module_image_name])
        .args(["--build-arg", &format!("ARCH={}", arch)])
        .args(["--build-arg", &format!("KERNEL_VERSION={}", kernel_version)])
        .args(["--build-arg", &format!("MODULE_VERSION={}", module.version)])
        .args(["--build-arg", &format!("PODMOD_VERSION={}", podmod_version)]);

    // Add additional build parameter passed to the function
    for (key, value) in &module.build_args {
        command.args(["--build-arg", &format!("{}={}", key, value)]);
    }

    command
        .arg(format!("{}/modules/{}", &config.data_dir, module.name))
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

pub fn load(module: &config::ModuleConfig, idempotent: bool) {
    // Check if module is already loaded
    if fetch::is_module_loaded(&module.name) {
        if idempotent {
            return;
        }

        panic!("Module {} is already loaded", module.name);
    }

    // Check if Secure Boot is enabled
    if fetch::is_secure_boot_enabled() {
        panic!("Cannot load unsigned kernel modules if Secure Boot is enabled");
    }

    println!("Loading module {} ...", module.name);

    let mut command = vec![String::from("load")];
    command.extend(module.kernel_args.clone());

    // Call the load script inside a new container
    // Add additional kernel parameters passed to the function
    run(module, &command);
}

pub fn modules(config: &config::Config) {
    println!("The following kernel modules are supported:");

    // Each supported module has a subdirectory in 'data_dir'
    let modules = fs::read_dir(format!("{}/modules", config.data_dir))
        .expect("Error while reading data directory");

    for module in modules {
        // Print the path's basename
        let module = module.unwrap().path();
        let module = module.file_name().unwrap();
        let module = module.to_str().unwrap();
        println!("{}", module);
    }
}

pub fn run(module: &config::ModuleConfig, command: &Vec<String>) {
    // podmod's container images are always named predictably
    let kernel_version = fetch::kernel_version();
    let image_name = get_module_image_identifier(&module.name, &module.version, &kernel_version);

    // Ensure module is built
    if !image_exists(&image_name) {
        panic!("Module {} is not built", module.name);
    }

    println!("Executing command {:?}, in module {} ...", command, module.name);

    // Run the command inside a new container
    // Add additional Podman arguments from module configuration to the function
    process::Command::new("podman")
        .args(["run", "--rm", "--privileged"])
        .args(&module.container_args)
        .arg(&image_name)
        .args(command)
        .status()
        .expect("Error while loading the kernel module");
}

pub fn shell(module: &config::ModuleConfig, shell: &str) {
    let mut module = module.clone();
    module.container_args.extend(vec![String::from("-it")]);

    println!("Starting shell session in module {} ...", module.name);

    // Call the load script inside a new container
    // Add additional kernel parameters passed to the function
    run(&module, &vec![String::from(shell)]);
}

pub fn unload(module: &config::ModuleConfig, idempotent: bool) {
    // Check if module is loaded
    if !fetch::is_module_loaded(&module.name) {
        if idempotent {
            return;
        }

        panic!("Module {} is not loaded", module.name);
    }

    // podmod's container images are always named predictably
    let kernel_version = fetch::kernel_version();
    let image_name = get_module_image_identifier(&module.name, &module.version, &kernel_version);

    println!("Unloading module {} ...", module.name);

    // Call the unload script inside a new container
    process::Command::new("podman")
        .args(["run", "--rm", "--privileged", &image_name, "unload"])
        .status()
        .expect("Error while unloading the kernel module");
}
