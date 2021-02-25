#!/bin/bash

# Check root permission
if [ $EUID -ne 0 ]; then
        echo Root permission required! Run with sudo or login as root.
        exit 1
fi

# Helper function for installing
download_and_build() {
        if wget -qO- $1 | tar -zxf- && pushd ./$2; then
                ./autogen.sh && ./configure && make install
                popd && rm -rf ./$2
        fi
}

# Install system dependencies
apt-get install -y build-essential autoconf libtool pkg-config python3-pip && pip3 install cython
# Install zimg (from source)
download_and_build https://github.com/sekrit-twc/zimg/archive/release-2.9.3.tar.gz zimg-release-2.9.3
# Install vapoursynth (from source)
download_and_build https://github.com/vapoursynth/vapoursynth/archive/2f0a78495608424019b7a85510699ef68d8484c2.tar.gz 2f0a78495608424019b7a85510699ef68d8484c2
# Fix vapoursynth (native) python path
PYTHON3_LOCAL_LIB_PATH=$(echo /usr/local/lib/python3.*)
ln -s $PYTHON3_LOCAL_LIB_PATH/site-packages/vapoursynth.so $PYTHON3_LOCAL_LIB_PATH/dist-packages/vapoursynth.so
# Load vapoursynth into system libraries cache
ldconfig /usr/local/lib

# Test installation
python3 -c "from vapoursynth import core;print(core.version())"