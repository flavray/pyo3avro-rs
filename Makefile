.PHONY: all build build-release clean dev-packages fast-py-test py-test rust-test test

all: build

dev-packages:
	pipenv install --dev

build: dev-packages
	pipenv run pyo3-pack build

build-release: dev-packages
	pipenv run pyo3-pack build --release

install: dev-packages
	pipenv run pyo3-pack develop

test: rust-test py-test

rust-test:
	cargo test

py-test: dev-packages install fast-py-test

fast-py-test:
	pipenv run py.test tests

clean:
	pipenv --rm || true
	cargo clean
