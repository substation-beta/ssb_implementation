#!/bin/bash

# Check root permission
if [ $EUID -ne 0 ]; then
        echo Root permission required! Run with sudo or login as root.
        exit 1
fi

# Install kcov requirements
apt-get install -y libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev libiberty-dev
# Download & unpack kcov sources
wget -qO- https://github.com/SimonKagstrom/kcov/archive/master.tar.gz | tar -zxf-
# Move into intermediate kcov build directory
pushd kcov-master && mkdir build && cd build
# Build & install kcov
cmake .. && make install
# Delete kcov sources
popd && rm -rf kcov-master
# Generate code coverage reports
for file in target/debug/*_*-*[^\.d];do
	mkdir -p "target/cov/$(basename $file)"
	kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done

# Upload code coverage reports (by environment variable CODECOV_TOKEN)
bash <(curl -s https://codecov.io/bash)