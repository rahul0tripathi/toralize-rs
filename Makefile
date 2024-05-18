build:
	@ cargo build

run:
	@ cargo run --bin toralized

build-client:
	@ mkdir -p .build
	@ gcc toralize/connect.c -o .build/sample

run-client: build-client
	@ toralize/toralize.sh .build/sample
