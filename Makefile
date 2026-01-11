.PHONY: setup build-core build-modules clean run-core

setup:
	bash scripts/setup.sh

build-core:
	cd src && cargo build

build-modules:
	cd modules/neurospoit && go build -o neurospoit cmd/neurospoit/main.go
	# Ajouter les autres modules ici

run-core:
	cd src && cargo run

clean:
	rm -rf target/
	find . -name "*.exe" -delete
	find . -name "node_modules" -type d -exec rm -rf {} +
