.PHONY: doc
doc:
	cargo doc --no-deps

.PHONY: codegen
codegen:
	cd codegen && ./build.sh

.PHONY: test
test:
	cargo test --all-targets --all-features
	cargo test --doc

.PHONY: lint
lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -Dclippy::all
