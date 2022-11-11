#!/bin/bash

set -e

maturin build

pip3 install --force-reinstall ../target/wheels/py_tritium_remote-0.1.0-cp310-cp310-linux_x86_64.whl

