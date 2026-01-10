.PHONY: build-core build-modules test clean

build-core:
	cargo build -p aether-core

build-modules:
	cd modules/neurospoit && go build ./...
	cd modules/api-nemesis && go build ./...
	# Python modules require poetry
	cd modules/quantummapper && poetry build
	cd modules/auto-pentest && poetry build

test:
	cargo test
	cd modules/neurospoit && go test ./...

clean:
	cargo clean
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "node_modules" -type d -exec rm -rf {} +
