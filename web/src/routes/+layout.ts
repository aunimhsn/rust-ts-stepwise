// Every page is prerendered as an empty shell and drawn in the browser: the website is static,
// and the core only runs there, in WebAssembly (see docs/technical-choices.md).
export const ssr = false;
export const prerender = true;
