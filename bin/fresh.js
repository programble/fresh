#!/usr/bin/env node
'use strict';

let Promise = require('bluebird');

let oauth = require('../lib/oauth')
let gmail = require('../lib/gmail')

let auth = oauth.authorize();
let unread = auth.then(gmail.listUnread);
let messages = Promise.join(auth, unread, gmail.getMessages);

messages.then(console.log);
