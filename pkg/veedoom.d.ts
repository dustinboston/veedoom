/* tslint:disable */
/* eslint-disable */
/**
* @param {string} tag 
* @param {any} props_val 
* @param {any} children_val 
* @returns {any} 
*/
export function v(tag: string, props_val: any, children_val: any): any;
/**
* @param {any} current_val 
* @param {any} other_val 
* @returns {any} 
*/
export function update(current_val: any, other_val: any): any;
/**
*/
export function main_js(): void;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        