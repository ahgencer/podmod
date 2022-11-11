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

use toml;
use std::collections;
use std::fs;

#[derive(Clone, Debug)]
pub struct Config {
    pub data_dir: String,
    pub tree: toml::Value,
}

#[derive(Clone, Debug)]
pub struct ModuleConfig {
    pub name: String,
    pub version: String,
    pub container_args: Vec<String>,
    pub kernel_args: Vec<String>,
    pub build_args: collections::HashMap<String, String>,
}

pub fn parse(path: &str) -> Config {
    // Read file into String
    let file = fs::read_to_string(path)
        .expect(&format!("Error while reading configuration file at {}", path));

    // Parse file using the 'toml' crate
    let config = file
        .parse::<toml::Value>()
        .expect(&format!("Error while parsing configuration file at {}", path));

    // Fetch TOML values
    let data_dir = config
        .get("data_dir")
        .expect("Missing configuration option 'data_dir'")
        .as_str()
        .expect("Configuration option 'data_dir' must have a string value");

    let data_dir = String::from(data_dir);

    Config {
        data_dir,
        tree: config,
    }
}

pub fn module(config: &toml::Value, module: &str) -> ModuleConfig {
    // Fetch parent TOML tables
    let config = config
        .get(module)
        .expect(&format!("Missing configuration for {} module", module))
        .as_table()
        .expect(&format!("Configuration for {} module must be a table", module));

    let build_config = config
        .get("build")
        .expect(&format!("Missing build configuration for {} module", module))
        .as_table()
        .expect(&format!("Build configuration for {} module must be a table", module));

    // Fetch TOML values
    let name = String::from(module);

    let version = config
        .get("version")
        .expect(&format!("No version specified for {} module", module))
        .as_str()
        .expect(&format!("Version identifier for {} module must have a string value", module));

    let version = String::from(version);

    let container_args = toml::value::Value::try_from(Vec::<String>::new()).unwrap();
    let container_args = config
        .get("container_args")
        .unwrap_or(&container_args)
        .as_array()
        .expect(&format!("Container arguments for {} module must be an array", module));

    let msg = format!("Container argument for {} module must have a string value", module);
    let container_args: Vec<_> = container_args
        .iter()
        .map(|v| v.as_str().expect(&msg))
        .map(|v| String::from(v))
        .collect();

    let kernel_args = toml::value::Value::try_from(Vec::<String>::new()).unwrap();
    let kernel_args = config
        .get("kernel_args")
        .unwrap_or(&kernel_args)
        .as_array()
        .expect(&format!("Kernel parameters for {} module must be an array", module));

    let msg = format!("Kernel parameter for {} module must have a string value", module);
    let kernel_args: Vec<_> = kernel_args
        .iter()
        .map(|v| v.as_str().expect(&msg))
        .map(|v| String::from(v))
        .collect();

    let mut build_args = collections::HashMap::new();

    for (key, value) in build_config {
        let value = value
            .as_str()
            .expect(&format!("Build parameter for {} module must have a string value", module));

        let key = String::from(key);
        let value = String::from(value);

        build_args.insert(key, value);
    }

    ModuleConfig {
        name,
        version,
        container_args,
        kernel_args,
        build_args,
    }
}
