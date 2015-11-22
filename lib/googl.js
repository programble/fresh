'use strict';

let Promise = require('bluebird');
let google = require('googleapis');

let credentials = require('./credentials');

let googl = google.urlshortener('v1');

Promise.promisifyAll(googl.url);

// Shorten a URL with goo.gl.
function shorten(url) {
  return googl.url.insertAsync({
    key: credentials.API_KEY,
    resource: { longUrl: url },
  }).get('id');
}

module.exports = {
  shorten,
};
