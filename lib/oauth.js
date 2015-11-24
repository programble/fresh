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

// Read a single line of input from stdin.
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

// Prompt the user to open an auth URL for client and enter the code, returning
// a set of tokens.
function promptTokens(client) {
  return Promise.join(client, client =>
    Promise.resolve(
      client.generateAuthUrl({
        access_type: 'offline',
        scope: credentials.SCOPES,
      })
    )
    .then(googl.shorten)
    .then(url =>
      console.log(`Authorize by opening ${url} and pasting the code below.`)
    )
    .then(readLine)
    .then(code => client.getTokenAsync(code))
  );
}

// Create an authorized client, either by loading saved credentials or by
// prompting the user.
//
// New tokens are saved to a file.
function authorize() {
  let client = Promise.try(() =>
    new google.auth.OAuth2(
      credentials.CLIENT_ID,
      credentials.CLIENT_SECRET,
      credentials.REDIRECT_URI
    )
  );

  return fs.readFileAsync(tokensPath, { encoding: 'utf-8' })
    .then(JSON.parse)
    .catch({ code: 'ENOENT' }, () => promptTokens(client))
    .tap(tokens => client.call('setCredentials', tokens))
    .then(JSON.stringify)
    .then(json => fs.writeFileAsync(tokensPath, json))
    .return(client);
}

// Remove saved credentials, if they exist.
function unauthorize() {
  return fs.unlinkAsync(tokensPath)
    .catch({ code: 'ENOENT' });
}

module.exports = {
  authorize,
  unauthorize,
};
