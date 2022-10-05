Name:           podmod
Version:        0.2.0
Release:        1%{?dist}
Summary:        Containerized build system for kernel modules on Fedora

License:        GPLv2+
URL:            https://github.com/ahgencer/podmod
Source0:        %{name}-%{version}.tar.gz

BuildArch:      noarch

Requires:       podman

%description
Builds a kernel module from source inside a Podman container.
Targeted for Fedora Silverblue / Kinoite, but should also work for other
editions.

%prep
%autosetup
# Set configuration defaults
sed -i -E 's|^(readonly D_MODULES_DIR)=.+$|\1="%{_datadir}/%{name}/modules"|gm' src/%{name}

%build
# podmod is a shell script

%install
mkdir -p %{buildroot}%{_sbindir} %{buildroot}%{_datadir}/%{name}/ %{buildroot}%{_sysconfdir}
mkdir -p %{buildroot}%{_mandir}/man8/ %{buildroot}%{_mandir}/man5/
install -p -m0755 src/%{name} %{buildroot}%{_sbindir}
cp -rp lib/modules/ %{buildroot}%{_datadir}/%{name}/
install -p -m0755 extra/%{name}.conf %{buildroot}%{_sysconfdir}
install -p -m0644 docs/*.8 %{buildroot}%{_mandir}/man8/
install -p -m0644 docs/*.5 %{buildroot}%{_mandir}/man5/

%files
%license COPYING
%{_sbindir}/%{name}
%{_datadir}/%{name}/
%{_sysconfdir}/%{name}.conf
%{_mandir}

%changelog
* Wed Oct 05 2022 Alpin H. Gencer <ah@gencer.us> - 0.2.0-1
- Initial version
