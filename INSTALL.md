# Installation

*podmod* is available as a [Copr](https://docs.fedoraproject.org/en-US/infra/sysadmin_guide/copr/) repository
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

> **Note:** *podmod* will not work when it is installed inside a container, as the Podman commands will fail.

## Building from source

*podmod* is built as an RPM package using [Tito](https://github.com/rpm-software-management/tito). To build the package
from source, first install `tito` and other build dependencies (perhaps inside
a [Toolbx](https://docs.fedoraproject.org/en-US/fedora-silverblue/toolbox/) container):

    $ dnf install tito rust-packaging
    $ dnf install \
        rust-clap+default-devel rust-clap+derive-devel \
        rust-nix+default-devel \
        rust-toml+default-devel

Then build the latest stable version of the package with:

    # tito build -o dist/ [--test] --rpm

The locally built RPM and SRPM packages will be inside the `dist/` directory.

You can then install the package with one of:

    $ dnf install <PATH>
    $ rpm-ostree install <PATH>

Where `PATH` is the path to the generated RPM file.
