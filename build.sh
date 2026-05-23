#!/usr/bin/env bash
set -euo pipefail

# CF Pages uses shallow clones where git submodule update is a no-op;
# fall back to a direct clone when the directory is empty.
git submodule update --init --recursive || true
[ -f "external/company/build.py" ] || \
  git clone https://github.com/ThierryBerger/company.git external/company

pip install marimo uv

(cd external/company && python build.py)

mkdir -p static/research/company
cp -r external/company/build/. static/research/company/

zola build
