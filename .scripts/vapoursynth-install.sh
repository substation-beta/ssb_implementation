#!/bin/bash

# Check root permission
if [ $EUID -ne 0 ]; then
        echo Root permission required! Run with sudo or login as root.
        exit 1
fi

# Helper functions
download_and_build() {
        if wget -qO- $1 | tar -zxf- && pushd ./$2; then
                ./autogen.sh && ./configure && make install
                popd && rm -rf ./$2
        fi
}

# Install system dependencies
apt-get install -y build-essential autoconf libtool pkg-config python3-pip && pip3 install cython
# Install nasm (from source) [new version required]
download_and_build https://www.nasm.us/pub/nasm/releasebuilds/2.14.02/nasm-2.14.02.tar.gz nasm-2.14.02
# Install zimg (from source)
download_and_build https://github.com/sekrit-twc/zimg/archive/release-2.9.1.tar.gz zimg-release-2.9.1
# Install vapoursynth (from source)
download_and_build https://github.com/vapoursynth/vapoursynth/archive/R46.tar.gz vapoursynth-R46
# Load vapoursynth into system libraries cache
ldconfig /usr/local/lib
# Fix vapoursynth (native) python path
PYTHON3_LOCAL_LIB_PATH=$(echo /usr/local/lib/python3.*)
ln -s $PYTHON3_LOCAL_LIB_PATH/site-packages/vapoursynth.so $PYTHON3_LOCAL_LIB_PATH/dist-packages/vapoursynth.so

# Test installation
python3 -c "from vapoursynth import core;print(core.version())"