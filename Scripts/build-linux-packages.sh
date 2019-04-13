#!/bin/bash

make build
version="$(.build/release/tre -v)"
package_name=".build/tre_${version}-1"
control_file="${package_name}/DEBIAN/control"
rm -rf "${package_name}"
mkdir -p "${package_name}/usr/local/bin"
cp .build/release/tre "${package_name}/usr/local/bin/"
mkdir "${package_name}/DEBIAN"
touch "${control_file}"
echo "Package: tre"                             >> "${control_file}"
echo "Version: ${version}-1"                    >> "${control_file}"
echo "Section: base"                            >> "${control_file}"
echo "Priority: optional"                       >> "${control_file}"
echo "Architecture: amd64"                      >> "${control_file}"
echo "Maintainer: Daniel Duan <daniel@duan.ca>" >> "${control_file}"
echo "Description: An improved tree."           >> "${control_file}"
dpkg-deb --build ${package_name}
apt-get update
apt-get -y install alien
alien -r ${package_name}.deb
mv ./*.rpm .build
mv ${package_name}.deb ${package_name}.x86_64.deb
