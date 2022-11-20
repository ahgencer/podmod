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

use std::process;
use std::str;

pub fn architecture() -> String {
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

pub fn kernel_version() -> String {
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

pub fn is_module_loaded(module: &str) -> bool {
    // Call 'lsmod | grep $module' to check for loaded module
    let lsmod = process::Command::new("lsmod")
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Error while fetching loaded kernel modules");

    let mut grep = process::Command::new("grep")
        .arg(module)
        .stdin(process::Stdio::from(lsmod.stdout.unwrap()))
        .stdout(process::Stdio::null())
        .spawn()
        .expect("Error while searching for loaded kernel module");

    // 'grep' succeeds only if string is found
    grep.wait().unwrap().success()
}

pub fn is_secure_boot_enabled() -> bool {
    // Call 'mokutil --sb-state | grep enabled' to check for Secure Boot state
    let mokutil = process::Command::new("mokutil")
        .arg("--sb-state")
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Error while fetching Secure Boot state");

    let mut grep = process::Command::new("grep")
        .arg("enabled")
        .stdin(process::Stdio::from(mokutil.stdout.unwrap()))
        .stdout(process::Stdio::null())
        .spawn()
        .expect("Error while determining Secure Boot state");

    // 'grep' succeeds only if string is found
    grep.wait().unwrap().success()
}
