<h1 align="center">Podmod</h1>

<p align="center">
    <img alt="crates.io Downloads" src="https://img.shields.io/crates/d/podmod?label=crates.io%20Downloads">
    <img alt="GitHub Stars" src="https://img.shields.io/github/stars/ahgencer/podmod?label=GitHub%20Stars">
    <img alt="Issues" src="https://img.shields.io/github/issues/ahgencer/podmod/open?label=Issues">
    <img alt="License" src="https://img.shields.io/github/license/ahgencer/podmod?label=License">
</p>

- GitHub: https://github.com/ahgencer/podmod
- crates.io: https://crates.io/crates/podmod
- Issues: https://github.com/ahgencer/podmod/issues

*podmod provides a containerized method for building kernel modules on Fedora, mainly targeting immutable operating
systems such as Silverblue / Kinoite and CoreOS.*

*podmod* builds kernel modules from source inside a [Podman](https://podman.io/) container and allows you to load it
without modifying any part of the filesystem on the host. It provides a small POSIX frontend to source the build steps
of a module as a Containerfile, and to load and unload the module. The process is:

- You call `podmod build` with the name of the kernel module.
- *podmod* searches `share/modules/` for the module, sources the `manifest.sh` file, and builds the kernel module as
  part of a new container image.
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

### Will this work on my favorite edition of Fedora?

This has only been tested on Silverblue / Kinoite 36, but **will theoretically work** on other editions as well,
including Workstation, Server, and CoreOS. Think of it as an alternative to [dkms(8)](https://manpages.org/dkms/8), for
cases where the module in question is either not packages for Fedora yet, or when the root filesystem is not writable.

### Wil this work on distributions other than Fedora?

**No.** The modules are built against Fedora's kernel packages from [Koji](https://koji.fedoraproject.org/koji/) and are
incompatible with others. This restriction also includes distributions that are downstream from Fedora, such
as [CentOS](https://centos.org/) and [RHEL](https://redhat.com/en/technologies/linux-platforms/enterprise-linux).

You are welcome to adapt *podmod* to use different Containerfiles targeting other distributions, though!

## Getting started

### Installation

*podmod* is available as a [COPR](https://docs.fedoraproject.org/en-US/infra/sysadmin_guide/copr/) repository
at [ahgencer/podmod](https://copr.fedorainfracloud.org/coprs/ahgencer/podmod/).

On `dnf` based editions (Workstation, Server, etc.), you can install it the usual way with:

    $ dnf copr enable ahgencer/podmod
    $ dnf install podmod

On `rpm-ostree` based editions (Silverblue / Kinoite, CoreOS, etc.), you first need to add the `.repo` file
to `/etc/yum.repos.d/`:

    $ VERSION_ID=<VERSION>
    $ wget -P /etc/yum.repos.d/ "https://copr.fedorainfracloud.org/coprs/ahgencer/podmod/repo/fedora-$VERSION_ID/ahgencer-podmod-fedora-$VERSION_ID.repo"
    $ rpm-ostree install --apply-live podmod

Where `VERSION` is your Fedora version, as defined in `/etc/os-release` (eg. `36` or `rawhide`).

### Building from source

*podmod* is built as an RPM package using [Tito](https://github.com/rpm-software-management/tito). To build the package
yourself, install `tito` (perhaps inside a [Toolbx](https://docs.fedoraproject.org/en-US/fedora-silverblue/toolbox/)
container) and run:

    # tito build -o dist/ --rpm

The locally built RPM and SRPM packages will be inside the `dist/` directory.

You can then install the package with one of:

    $ dnf install <PATH>
    $ rpm-ostree install <PATH>

Where `PATH` is the path to the generated RPM file.

### Usage

To get help on using *podmod*, run:

    # podmod --help

You may also refer to the manpage [podmod(8)](docs/podmod.8).

To build a kernel module, run:

    $ podmod -m <MODULE> build

Afterwards, you can load it with:

    $ podmod -m <MODULE> load

## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public
License as published by the Free Software Foundation, either version 2 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied
warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not,
see <https://www.gnu.org/licenses/>.
