
# rustpylib

Demo of Rust-Python binding, running Rust library code from a Python application

## Explanation

There are 3 parts to this repo:
1. `rustlib` a normal rust library, built like any normal rust library (with `cargo new rustlib --lib`)
2. `rustpylib` a rust-python binding library,
built with [`maturin new`](https://github.com/PyO3/maturin?tab=readme-ov-file)
leveraging [`pyo3`](https://github.com/PyO3/pyo3?tab=readme-ov-file)
and using `rustlib` for its functions
3. `pybin.py` a normal python application that uses the `rustpylib` functions like any other python library

The only modifications from the defaults provided by these commands (beyond adding the desired functions)
are in `rustpylib`'s `Cargo.toml` to add dependency `rustlib`
and to add feature [`abi3-py38`](https://www.maturin.rs/tutorial#create-a-new-rust-project) to dep `pyo3`

## Setup

Need installed: python3, pipx, virtualenv, maturin -- and a virtualenv
```
sudo apt install -y python3 python3-pip pipx # ubuntu

pipx ensurepath
pipx install virtualenv
pipx install maturin # for PyO3 rust ffi into python

virtualenv .venv
```

## Running

You must have an active virtualenv so maturin can register the new rustpylib in this venv for python apps:
```
source .venv/bin/activate
```

Build the `rustpylib` and register it in the venv for `pybin.py` to use in a moment
```
cd rustpylib
maturin develop

cd ..
python3 pybin.py
```

Enter your name and 2 integers, per the prompts. You have run rust code from a python application!
