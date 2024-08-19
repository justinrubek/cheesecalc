/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_wasm(): void;
/**
* @param {number} cheese_mass
* @returns {any}
*/
export function from_cheese_mass(cheese_mass: number): any;
/**
* @param {number} pasta_mass
* @returns {any}
*/
export function from_pasta_mass(pasta_mass: number): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_wasm: () => void;
  readonly from_cheese_mass: (a: number, b: number) => void;
  readonly from_pasta_mass: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
