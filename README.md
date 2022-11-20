<h1 align="center">Podmod</h1>

<p align="center">
    <a href="https://copr.fedorainfracloud.org/coprs/ahgencer/podmod/">
        <img alt="Copr" src="https://img.shields.io/badge/Copr-ahgencer%2Fpodmod-51a2da">
    </a>
    <a href="https://crates.io/crates/podmod">
        <img alt="crates.io Downloads" src="https://img.shields.io/crates/d/podmod?label=crates.io%20Downloads">
    </a>
    <a href="https://github.com/ahgencer/podmod">
        <img alt="GitHub Stars" src="https://img.shields.io/github/stars/ahgencer/podmod?label=GitHub%20Stars">
    </a>
    <br>
    <a href="https://github.com/ahgencer/podmod/issues">
        <img alt="Issues" src="https://img.shields.io/github/issues/ahgencer/podmod/open?label=Issues">
    </a>
    <a href="https://github.com/ahgencer/podmod#license">
        <img alt="License" src="https://img.shields.io/github/license/ahgencer/podmod?label=License">
    </a>
    <img alt="Community Built" src="https://img.shields.io/badge/Made%20with-%E2%9D%A4-red">
</p>

*podmod provides a containerized method for building kernel modules on Fedora, mainly targeting immutable operating
systems such as Silverblue / Kinoite and CoreOS.*

*podmod* builds kernel modules from source inside a [Podman](https://podman.io/) container and allows you to load it
without modifying any part of the filesystem on the host. It provides a [Rust](https://rust-lang.org/) frontend that can
sources the build steps of a module from a Containerfile, and then load and unload the module. The process is:

- You call `podmod build` with the name of the kernel module.
- *podmod* reads the configuration file (default: `/etc/podmod.conf`) for build and kernel arguments.
- *podmod* searches `share/modules/` for the module and builds it as part of a new container image.
- You can then load or unload the module with `podmod load` or `podmod unload`. *podmod* will
  call [insmod(8)](https://manpages.org/insmod/8) or [rmmod(8)](https://manpages.org/rmmod/8) from **inside** the
  container to load or unload the module on the host.

Interested? [Here's how to get started.](#getting-started)

## FAQ

### Isn't this super hacky?

**Not really.** Containers aren't virtual machines, where the guest operating system has its own kernel, gets assigned
its own memory space to manage, and may be completely unaware that it's being virtualized. Instead, container engines
such as Podman or [Docker](https://docker.com/) use [Linux namespaces](https://en.wikipedia.org/wiki/Linux_namespaces)
to make a sort of [chroot(1)](https://manpages.org/chroot) with an isolated process and network space. Otherwise, its no
different from running the same command directly on the host. The kernel module is built the same way, and the kernel is
the same inside and outside the container.

Building kernel modules this way is not a brand-new concept,
either. [jdoss/atomic-wireguard](https://github.com/jdoss/atomic-wireguard) takes the same approach. There's even
an [article](https://projectatomic.io/blog/2018/06/building-kernel-modules-with-podman/) on building kernel modules with
Podman on the [Project Atomic](https://projectatomic.io/) website (which is now deprecated in favor of CoreOS). However,
the usual restrictions for kernel modules still apply. Mainly, the module needs to be built for a **specific** kernel
version, and must be rebuilt with every update.

### Will this work on other editions of Fedora?

This has only been tested on Silverblue / Kinoite (36 to 37), but **will theoretically work** on other editions as well,
including Workstation, Server, and CoreOS. Think of it as an alternative to [dkms(8)](https://manpages.org/dkms/8), for
cases where the module in question is either not packages for Fedora yet, or when the root filesystem is not writable.

### Wil this work on distributions other than Fedora?

**No.** The modules are built against Fedora's kernel packages from [Koji](https://koji.fedoraproject.org/koji/) and are
incompatible with other distributions. This restriction also excludes distributions that are downstream from Fedora,
such as [CentOS](https://centos.org/) and [RHEL](https://redhat.com/en/technologies/linux-platforms/enterprise-linux).

You are welcome to adapt *podmod* to use different Containerfiles targeting other distributions, though!

## Getting started

### Installation

Installation instructions, as well as instructions for building *podmod* from source, can be found [here](INSTALL.md).

### Basic Usage

To get help on using *podmod*, run:

    # podmod --help

You may also refer to the manpage [podmod(8)](docs/podmod.8).

To build a kernel module, run:

    $ podmod build -m <MODULE>

Afterwards, you can load it with:

    $ podmod load -m <MODULE>

*podmod* also ships with a [systemd](https://systemd.io/) service file to load and unload a module at boot time:

    $ systemctl enable podmod@<MODULE>.service

> **Note:** The module must have already been built manually on the system using `podmod build`. Otherwise, the unit
> will fail.

## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public
License as published by the Free Software Foundation, either version 2 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not,
see <https://www.gnu.org/licenses/>.
