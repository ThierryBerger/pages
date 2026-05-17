#!/usr/bin/env bash
set -euo pipefail

git submodule update --init --recursive

pip install marimo

(cd external/company && python build.py)

mkdir -p static/research/company
cp -r external/company/build/. static/research/company/

zola build
