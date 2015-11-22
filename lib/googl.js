'use strict';

let google = require('googleapis');

let credentials = require('./credentials');

let googl = google.urlshortener('v1');

function shorten(url) {
  return new Promise((resolve, reject) =>
    googl.url.insert({
      key: credentials.API_KEY,
      resource: { longUrl: url },
    }, (err, response) => err
      ? reject(err)
      : resolve(response.id)
    )
  );
}

module.exports = {
  shorten: shorten,
};
