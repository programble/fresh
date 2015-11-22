'use strict';

let google = require('googleapis');

let credentials = require('./credentials');

let googl = google.urlshortener('v1');

function shorten(url, done) {
  googl.url.insert({
    key: credentials.API_KEY,
    resource: { longUrl: url },
  }, (err, response) => err
    ? done(err)
    : done(null, response.id)
  );
}

module.exports = {
  shorten: shorten,
};
