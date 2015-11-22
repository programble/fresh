'use strict';

let crypto = require('crypto');

let Promise = require('bluebird');

Promise.promisifyAll(crypto);

function generate(len) {
  return crypto.randomBytesAsync(len)
    .then(buf => buf.toString('base64'))
    .then(str => str.slice(0, len));
}

module.exports = {
  generate: generate,
};
