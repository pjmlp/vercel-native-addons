import { createRequire } from "module";
const require = createRequire(import.meta.url);

// import the native module
const  hellolib = require("./lib/hello");

/**
 * @param {string} msg A message to output
 * @returns {string} 
 */
export function sayHello(msg) {

    return `${msg} ${hellolib.hello()}`;
}