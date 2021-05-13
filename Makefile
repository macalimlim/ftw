all:
	cargo build

audit:
	cargo audit

bloat:
	cargo bloat --release -n 10
	cargo bloat --release --crates
	cargo bloat --release --filter '^__' -n 10

build:
	cargo build

build-release: clean
	cargo build --release

check:
	cargo check
	cargo clippy --release -- -Dclippy::all -Wclippy::pedantic

clean:
	cargo clean

coverage:
	cargo tarpaulin --all-features -o Html -t 120
	${BROWSER} tarpaulin-report.html

doc: clean
	cargo doc --no-deps --open -v

expand:
	cargo expand --lib ftw_node_type

format:
	cargo fmt --all -- --check

list-node-types:
	godot-headless --gdnative-generate-json-api api.json
	cat api.json | jq '.[] | .name' | tr -d '"' | sort | uniq
	rm api.json

outdated:
	cargo outdated -R

publish: clean format check test
	cargo package
	cargo publish

shell:
	nix-shell -p openssl pkgconfig

test:
	cargo test

udeps:
	cargo udeps
