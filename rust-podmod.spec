%bcond_without check
%global __cargo_skip_build 0

%global crate podmod

Name:           rust-%{crate}
Version:        0.3.0
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
License:        GPL-2.0-or-later

%description -n %{crate} %{_description}

%global debug_package %{nil}

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
mkdir -p %{buildroot}%{_datadir}/%{crate}/ %{buildroot}%{_sysconfdir}
mkdir -p %{buildroot}%{_mandir}/man8/ %{buildroot}%{_mandir}/man5/
cp -pr share/modules/ %{buildroot}%{_datadir}/%{crate}/
install -p -m0755 extra/%{crate}.conf %{buildroot}%{_sysconfdir}
install -p -m0644 docs/*.8 %{buildroot}%{_mandir}/man8/
install -p -m0644 docs/*.5 %{buildroot}%{_mandir}/man5/

%if %{with check}
%check
%cargo_test -a
%endif

%files
%license COPYING
%{_sbindir}/%{crate}
%{_datadir}/%{crate}/
%{_sysconfdir}/%{crate}.conf
%{_mandir}

%changelog
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
