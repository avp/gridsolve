ROOT=$(PWD)
PKG_DIR = $(ROOT)/$(WWW_DIR)/pkg/
JS_DIST_DIR = $(ROOT)/$(WWW_DIR)/dist/
WASM_FLAGS =--target web --release
WASM_CRATE = crates/gridsolve_wasm
WWW_DIR = www
ESBUILD_FLAGS = --sourcemap --bundle --target=firefox100,chrome100

wasm:
	wasm-pack build $(WASM_FLAGS) $(WASM_CRATE) --out-dir $(PKG_DIR)

www:
	esbuild $(ESBUILD_FLAGS) $(WWW_DIR)/app.jsx --outfile=$(JS_DIST_DIR)/dist.js

watch-www:
	esbuild $(ESBUILD_FLAGS) $(WWW_DIR)/app.jsx --outfile=$(JS_DIST_DIR)/dist.js --watch

all: wasm www
	mkdir -p $(JS_DIST_DIR)
	cp $(PKG_DIR)/gridsolve_wasm_bg.wasm $(JS_DIST_DIR)

release: all
	mkdir -p build
	cp -r $(WWW_DIR)/*.html $(WWW_DIR)/*.css $(WWW_DIR)/dist build

.PHONY: wasm wasm-release www all release
