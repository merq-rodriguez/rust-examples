var ffi = require('ffi');

var lib = ffi.Library(
  'target/release/libembeber.so', {
  'procesar': ['void', []],
  'suma': ['int', ['int', 'int']],
  'resta': ['int', ['int', 'int']]
});

lib.procesar();
console.log("completado!");

let suma = lib.suma(45, 5);
console.log("Suma: "+suma)

let resta = lib.resta(45, 5);
console.log("Resta: "+resta)