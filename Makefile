.PHONY: build
build:
	RUSTFLAGS="-L /opt/wasi-sdk/share/wasi-sysroot/lib/wasm32-wasip2" CC="/opt/wasi-sdk/bin/clang" CFLAGS="-Wno-implicit-function-declaration" cargo build --target=wasm32-wasip2

.PHONY: web-install
web-install:
	(cd web && npm install)

.PHONY: web-start
web-start: build web-install
	(cd web && npm run start)

.PHONY: web-build
web-build: build web-install
	(cd web && npm run build)

.PHONY: clean
clean:
	cargo clean
	(cd web && rm -rf node_modules generated)
