#!/usr/bin/env node
'use strict';

let oauth = require('../lib/oauth')

oauth.authorize()
  .then(console.log);
