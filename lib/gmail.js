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

// Get the contents of a message.
function getMessage(auth, message) {
  return gmail.users.messages.getAsync({
    auth: auth,
    userId: 'me',
    id: message.id,
  });
}

// Mark a message as read and remove it from the user's inbox.
function archiveMessage(auth, message) {
  return gmail.users.messages.modifyAsync({
    auth: auth,
    userId: 'me',
    id: message.id,
    resource: {
      addLabelIds: [],
      removeLabelIds: ['INBOX', 'UNREAD'],
    },
  });
}

module.exports = {
  listUnread: listUnread,
  getMessage: getMessage,
  archiveMessage: archiveMessage,
};
