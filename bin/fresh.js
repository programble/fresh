#!/usr/bin/env node
'use strict';

let oauth = require('../lib/oauth');
let gmail = require('../lib/gmail');

let auth = oauth.authorize();
let list = gmail.listUnread(auth);
let messages = list.map(m => gmail.getMessage(auth, m));

messages.then(console.log);
