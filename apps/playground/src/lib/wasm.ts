let wasmModule: typeof import("@markright/markright-wasm") | null = null;
let wasmPromise: Promise<typeof import("@markright/markright-wasm")> | null =
  null;

export async function getWasm() {
  if (wasmModule) {
    return wasmModule;
  }

  if (!wasmPromise) {
    wasmPromise = import("@markright/markright-wasm").then(async (mod) => {
      await mod.default(new URL("/markright_wasm_bg.wasm", window.location.origin));
      wasmModule = mod;
      return mod;
    });
  }

  return wasmPromise;
}
