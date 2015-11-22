#!/usr/bin/env node
'use strict';

let oauth = require('../lib/oauth')
let gmail = require('../lib/gmail')

oauth.authorize()
  .then(gmail.listUnread)
  .then(console.log);
