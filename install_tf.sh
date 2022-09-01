#!/bin/sh
if [ "$(uname -m)" == "aarch64" ]; then
    pip3 install tensorflow-aarch64
else
    pip3 install tensorflow
fi
