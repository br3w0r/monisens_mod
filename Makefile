.PHONY: module-bindings
module-bindings:
	$(info Generaing FFI bindings...)
	bindgen ../monisens/module/monisens_def.h \
		--default-enum-style rust \
		--allowlist-file '../monisens/module/monisens_def.h' \
		--raw-line '#![allow(warnings)]' \
		-o ./src/bindings_gen.rs

.PHONY: generate
generate: module-bindings
