.PHONY: build-rust-dev
build-rust-dev:
	@cd rust && cargo build && cd ..

.PHONY: build-bindings
build-bindings:
	@cd rust && maturin develop && cd ..
