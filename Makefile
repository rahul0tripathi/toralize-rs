.PHONY: build
build:
	@ cargo build

.PHONY: run
run:
	@ cargo run --bin toralized

.PHONY: build-client
build-client:
	@ mkdir -p .build
	@ gcc toralize/connect.c -o .build/sample -lcurl

.PHONY: run-client
run-client: build-client
	@ toralize/toralize.sh .build/sample
