Name:           podmod
Version:        0.3.0
Release:        1%{?dist}
Summary:        Containerized build system for kernel modules on Fedora

License:        GPLv2+
URL:            https://github.com/ahgencer/podmod
Source0:        %{name}-%{version}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  cargo
BuildRequires:  rust-packaging

Requires:       podman

%description
Builds a kernel module from source inside a Podman container.
Targeted for Fedora Silverblue / Kinoite, but also works for other editions.

%global debug_package %{nil}

%prep
%autosetup
%cargo_prep
%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build -a

%install
%cargo_install -a
mv %{buildroot}%{_bindir} %{buildroot}%{_sbindir}
mkdir -p %{buildroot}%{_datadir}/%{name}/ %{buildroot}%{_sysconfdir}
mkdir -p %{buildroot}%{_mandir}/man8/ %{buildroot}%{_mandir}/man5/
cp -pr share/modules/ %{buildroot}%{_datadir}/%{name}/
install -p -m0755 extra/%{name}.conf %{buildroot}%{_sysconfdir}
install -p -m0644 docs/*.8 %{buildroot}%{_mandir}/man8/
install -p -m0644 docs/*.5 %{buildroot}%{_mandir}/man5/

%check
%cargo_test -a

%files
%license COPYING
%{_sbindir}/%{name}
%{_datadir}/%{name}/
%{_sysconfdir}/%{name}.conf
%{_mandir}

%changelog
* Fri Oct 07 2022 Alpin H. Gencer <ah@gencer.us> 0.3.0-1
- Rewrite frontend script in Rust

* Thu Oct 06 2022 Alpin H. Gencer <ah@gencer.us> 0.2.2-1
- Initialize tito

* Wed Oct 05 2022 Alpin H. Gencer <ah@gencer.us> - 0.2.1-1
- Update project description

* Wed Oct 05 2022 Alpin H. Gencer <ah@gencer.us> - 0.2.0-1
- Initial version
