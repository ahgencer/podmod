#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#

## Stage 1: Setup build environment
FROM registry.fedoraproject.org/fedora:latest AS staging

ARG ARCH
ARG KERNEL_VERSION

# Install common build dependencies
RUN dnf install -y \
      autoconf \
      automake \
      gc \
      gcc \
      git \
      glibc-devel \
      glibc-headers \
      koji \
      libtool \
      make \
      ncompress \
      rpm-build

# Download and install correct kernel packages from koji
RUN mkdir -p /app/koji && \
    cd /app/koji && \
    koji download-build --arch=$ARCH kernel-$KERNEL_VERSION && \
    dnf install -y --setopt=install_weak_deps=False \
      ./kernel-core-$KERNEL_VERSION.rpm \
      ./kernel-devel-$KERNEL_VERSION.rpm \
      ./kernel-modules-$KERNEL_VERSION.rpm

# Minimize final image size
RUN dnf clean all -y

## Stage 2: Squash generated image layers
FROM scratch
COPY --from=staging / /

# Set default command for container image
CMD /usr/bin/bash
