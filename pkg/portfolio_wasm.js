/* @ts-self-types="./portfolio_wasm.d.ts" */
import * as wasm from "./portfolio_wasm_bg.wasm";
import { __wbg_set_wasm } from "./portfolio_wasm_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    run
} from "./portfolio_wasm_bg.js";
