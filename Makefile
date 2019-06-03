.PHONY: all build build-release clean dev-packages fast-py-test static-py-test py-test rust-test test venv

all: build

dev-packages:
	pipenv install --dev
	pipenv run pre-commit install -f --install-hooks

build: dev-packages
	pipenv run pyo3-pack build

build-release: dev-packages
	pipenv run pyo3-pack build --release

install: dev-packages
	pipenv run pyo3-pack develop

test: rust-test py-test

rust-test:
	cargo test --no-default-features

py-test: dev-packages install fast-py-test static-py-test

fast-py-test:
	pipenv run py.test tests

venv: dev-packages
	pipenv shell

static-py-test:
	pipenv run pre-commit run --all-files

clean:
	pipenv --rm || true
	cargo clean
	rm -rf dist
	find . -name '*.pyc' -delete
	find . -name '__pycache__' -delete
