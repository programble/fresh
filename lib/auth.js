'use strict';

let os = require('os');
let fs = require('fs');
let path = require('path');
let readline = require('readline');

let Promise = require('bluebird');
let google = require('googleapis');

let credentials = require('./credentials');
let googl = require('./googl');

Promise.promisifyAll(fs);
Promise.promisifyAll(google.auth.OAuth2.prototype);

let tokensPath = path.join(os.homedir(), '.fresh.tokens.json');

let client = new google.auth.OAuth2(
  credentials.CLIENT_ID,
  credentials.CLIENT_SECRET,
  credentials.REDIRECT_URI
);

function readLine() {
  return new Promise((resolve, reject) => {
    let rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });

    rl.prompt();

    rl.on('line', line => {
      rl.close();
      resolve(line);
    });
  });
}

function promptTokens() {
  let authURL = Promise.try(() =>
    client.generateAuthUrl({
      access_type: 'offline',
      scope: credentials.SCOPES,
    })
  );
  return authURL
    .then(googl.shorten)
    .tap(url =>
      console.log('Authorize by opening', url, 'and pasting the code below.')
    )
    .then(readLine)
    .then(code => client.getTokenAsync(code));
}

function authorize() {
  return fs.readFileAsync(tokensPath, { encoding: 'utf-8' })
    .then(JSON.parse)
    .catch({ code: 'ENOENT' }, () => promptTokens())
    .tap(tokens => client.setCredentials(tokens))
    .then(JSON.stringify)
    .then(json => fs.writeFileAsync(tokensPath, json));
}

module.exports = {
  authorize: authorize,
  client: client,
};
