'use strict';

let jsc = require('jsverify');

let password = require('../lib/password');

describe('password', () => {
  describe('generate', () => {
    jsc.property('length', jsc.integer(1, 1024), (n) =>
      password.generate(n).then(s => n === s.length)
    );
  });
});
