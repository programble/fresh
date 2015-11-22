#!/usr/bin/env node
'use strict';

let Promise = require('bluebird');

let oauth = require('../lib/oauth')
let gmail = require('../lib/gmail')

let auth = oauth.authorize();
let unread = auth.then(gmail.listUnread);
let messages = auth.then(auth => unread.map(m => gmail.getMessage(auth, m)));

messages.then(console.log);
