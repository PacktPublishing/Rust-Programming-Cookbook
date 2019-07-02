const ffi = require('ffi');
const ref = require('ref');

const libPath = '../rust-digest/target/debug/libdigest';


const libWeb = ffi.Library(libPath, {
  'digest': [ "string", ["string"]],
});

const { digest } = libWeb;
console.log('Hello World SHA256', digest("Hello World"));