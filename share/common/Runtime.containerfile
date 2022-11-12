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

## Stage 1: Setup common runtime
FROM registry.fedoraproject.org/fedora:latest AS staging

ARG KERNEL_VERSION
ARG PODMOD_VERSION

# Copy and install kernel packages from builder stage
COPY --from=podmod-builder:$PODMOD_VERSION-$KERNEL_VERSION /app/koji/ /tmp/koji/

RUN cd /tmp/koji && \
    dnf install -y --setopt=install_weak_deps=False \
      ./kernel-core-$KERNEL_VERSION.rpm \
      ./kernel-modules-$KERNEL_VERSION.rpm && \
    rm -rf /tmp/koji

# Minimize final image size
RUN dnf clean all -y

## Stage 2: Squash generated image layers
FROM scratch
COPY --from=staging / /

# Set default command for container image
CMD /usr/bin/bash
