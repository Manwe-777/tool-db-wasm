# ToolDb rust WASM extension

This is meant for usage within tooldb, an attempt to isolate any slow function into WASM.

Not really meant to be used yet, more of a proof of concept, as of right now it only includes the ECC recovery function for ETH addresses.

Build to wasm: 

```wasm-pack build --target web```

_* build might fail on Windows, but you can use Ubuntu/WSL_