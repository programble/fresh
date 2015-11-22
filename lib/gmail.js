'use strict';

let Promise = require('bluebird');
let google = require('googleapis');

let gmail = google.gmail('v1');

Promise.promisifyAll(gmail.users.messages);

// List unread messages in the user's inbox.
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

// Get the contents of an array of messages.
function getMessages(auth, messages) {
  return Promise.map(messages, (message) =>
    gmail.users.messages.getAsync({
      auth: auth,
      userId: 'me',
      id: message.id,
    })
  );
}

module.exports = {
  listUnread: listUnread,
  getMessages: getMessages,
};
