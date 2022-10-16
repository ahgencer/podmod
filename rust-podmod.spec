%bcond_without check
%global debug_package %{nil}

%global crate podmod

Name:           rust-%{crate}
Version:        0.3.2
Release:        3%{?dist}
Summary:        Containerized build system for kernel modules on Fedora
License:        GPL-2.0-or-later

URL:            https://crates.io/crates/%{crate}
Source0:        %{crates_source}

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging
Requires:       podman

%global _description %{expand:
Builds a kernel module from source inside a Podman container.
Targeted for Fedora Silverblue / Kinoite, but also works for other editions.}

%description %{_description}

%package     -n %{crate}
Summary:        %{summary}

%description -n %{crate} %{_description}

%files       -n %{crate}
%license COPYING
%doc README.md
%{_sbindir}/podmod
%{_datadir}/podmod/
%{_mandir}

%package        devel
Summary:        %{summary}

BuildArch:      noarch

%description    devel %{_description}

This package contains library source intended for building other packages which
use the "%{crate}" crate.

%files          devel
%license %{crate_instdir}/COPYING
%doc %{crate_instdir}/README.md
%{crate_instdir}/

%package     -n %{name}+default-devel
Summary:        %{summary}

BuildArch:      noarch

%description -n %{name}+default-devel %{_description}

This package contains library source intended for building other packages which
use the "default" feature of the "%{crate}" crate.

%files       -n %{name}+default-devel
%ghost %{crate_instdir}/Cargo.toml

%prep
%autosetup -n %{crate}-%{version_no_tilde} -p1
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires -a

%build
%cargo_build -a

%install
%cargo_install -a
mv %{buildroot}%{_bindir} %{buildroot}%{_sbindir}
mkdir -p %{buildroot}%{_datadir}/podmod/
mkdir -p %{buildroot}%{_mandir}/man8/
cp -pr share/modules/ %{buildroot}%{_datadir}/podmod/
install -p -m0644 docs/*.8 %{buildroot}%{_mandir}/man8/

%if %{with check}
%check
%cargo_test -a
%endif

%changelog
* Sun Oct 16 2022 Alpin H. Gencer <ah@gencer.us> 0.3.1-1
- Configuration file removed in version 0.3.1

* Sat Oct 15 2022 Alpin H. Gencer <ah@gencer.us> 0.3.0-5
- Create subpackages for split binary and library crates

* Thu Oct 13 2022 Alpin H. Gencer <ah@gencer.us> 0.3.0-4
- Bundle README as documentation in package

* Wed Oct 12 2022 Alpin H. Gencer <ah@gencer.us> 0.3.0-3
- Publish crate to crates.io
- Re-package RPM according to Fedora Rust Packaging Guidelines
- Re-create package podmod as subpackage of rust-podmod

* Wed Oct 12 2022 Alpin H. Gencer <ah@gencer.us> 0.3.0-2
- Fill in missing check condition from spec file generated with rust2rpm
- Use SPDX license identifier

* Fri Oct 07 2022 Alpin H. Gencer <ah@gencer.us> 0.3.0-1
- Rewrite frontend script in Rust

* Thu Oct 06 2022 Alpin H. Gencer <ah@gencer.us> 0.2.2-1
- Initialize tito

* Wed Oct 05 2022 Alpin H. Gencer <ah@gencer.us> - 0.2.1-1
- Update project description

* Wed Oct 05 2022 Alpin H. Gencer <ah@gencer.us> - 0.2.0-1
- Initial version
