'use strict';

let crypto = require('crypto');

let Promise = require('bluebird');

Promise.promisifyAll(crypto);

// Generate a random base64 password of length len.
function generate(len) {
  return crypto.randomBytesAsync(len)
    .then(buf => buf.toString('base64'))
    .then(str => str.slice(0, len));
}

module.exports = {
  generate: generate,
};
