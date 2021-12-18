.PHONY: build-rust-dev
build-rust:
	@cd rust && cargo build --release && cd ..

.PHONY: build-bindings
build-bindings:
	@cd rust && maturin develop --release && cd ..
