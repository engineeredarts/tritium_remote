#!/bin/bash

set -e

maturin build

pip3 install --force-reinstall ../target/wheels/tritium_remote-1.2.1-cp310-cp310-manylinux_2_34_x86_64.whl

