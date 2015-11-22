#!/usr/bin/env node
'use strict';

let auth = require('../lib/auth')

auth.authorize()
  .return(auth.client)
  .then(console.log);
