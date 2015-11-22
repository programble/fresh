'use strict';

let Promise = require('bluebird');
let google = require('googleapis');

let gmail = google.gmail('v1');

Promise.promisifyAll(gmail.users.messages);

function listUnread(auth) {
  let res = gmail.users.messages.listAsync({
    auth: auth,
    userId: 'me',
    labelIds: ['INBOX', 'UNREAD'],
  });

  return res
    .then(res =>
      res.resultSizeEstimate === 0
        ? []
        : res.messages
    );
}

module.exports = {
  listUnread: listUnread,
};
