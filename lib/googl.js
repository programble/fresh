'use strict';

let Promise = require('bluebird');
let google = require('googleapis');

let credentials = require('./credentials');

let googl = google.urlshortener('v1');
Promise.promisifyAll(googl.url);

function shorten(url) {
  return googl.url.insertAsync({
    key: credentials.API_KEY,
    resource: { longUrl: url },
  }).then(res => res.id);
}

module.exports = {
  shorten: shorten,
};
