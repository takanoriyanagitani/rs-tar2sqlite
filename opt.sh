iwasi=./rs-tar2sqlite.wasm

wasm-opt \
	-Oz \
	-o rs-tar2sqlite.opt.wasm \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	--enable-simd \
	"${iwasi}"
