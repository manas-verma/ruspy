.PHONY: all
all:
	@echo "Run my targets individually!"

.PHONY: env
env:
	test -d env || python3 -m venv env
	. env/bin/activate && \
		python -m pip install --upgrade pip && \
		python -m pip install -r test/requirements.txt && \
		python -m pip install maturin


.PHONY: develop
develop: env
	. env/bin/activate && \
		maturin develop

.PHONY: test
test: develop
	. env/bin/activate && \
		python -m unittest test/test_ruspy.py

.PHONY: build
build: env
	. env/bin/activate && \
		maturin build

.PHONY: install
install:
	pip3 install --force-reinstall target/wheels/*.whl

.PHONY: dist
dist: env
	. env/bin/activate && \
		docker run --rm -v $(shell pwd):/io ghcr.io/pyo3/maturin build --release --strip --out dist