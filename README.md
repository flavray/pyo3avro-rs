# pyo3avro-rs

[![GitHub version](https://badge.fury.io/gh/flavray%2Fpyo3avro-rs.svg)](https://badge.fury.io/gh/flavray%2Fpyo3avro-rs)
[![Build Status](https://travis-ci.org/flavray/pyo3avro-rs.svg?branch=master)](https://travis-ci.org/flavray/pyo3avro-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/flavray/pyavro-rs/blob/master/LICENSE)

CPython wrapper for the [avro-rs](https://github.com/flavray/avro-rs) library.
It uses the [pyo3](https://github.com/PyO3/pyo3) Rust framework to interact
with the CPython interpreter.

For more information on how the original avro-rs works, please have a look at
the [documentation](https://docs.rs/avro-rs).

## Installation

The installation currently requires [`cargo`](https://doc.rust-lang.org/cargo/)
to be installed (this will change in the future!)

Here are the steps to follow in order to run the test suite:

    # Install rustup - https://www.rust-lang.org/en-US/install.html
    $ curl https://sh.rustup.rs -sSf | sh

    # Install pipenv - https://docs.pipenv.org/en/latest/install/#installing-pipenv
    $ pip install --user pipenv

    # Clone pyo3avro_rs
    $ git clone https://github.com/flavray/pyo3avro-rs.git

    # Build the project
    $ cd pyo3avro_rs/
    $ make

    # Run the test suite
    $ make test


Here are the steps to follow in order to run an example:

    # Run the example
    $ pipenv run python examples/example.py

### Existing Rust installation

This project requires rust nightly >= 1.31 (see
[this comment](https://github.com/alexcrichton/rustc-demangle/issues/21#issuecomment-483350917))

You may have to force your rustup update if you come across the following error:

    Caused by: feature `rename-dependency` is required

    # Update rust nightly
    $ rustup update --force
    $ rustup override set nightly
