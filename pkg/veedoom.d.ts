/* tslint:disable */
/* eslint-disable */
/**
* @param {string} tag 
* @param {any} props_val 
* @param {any} children_val 
* @returns {any} 
*/
export function create_element(tag: string, props_val: any, children_val: any): any;
/**
* @param {any} node_val 
* @param {string | undefined} root_id 
*/
export function render(node_val: any, root_id?: string): void;
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
        