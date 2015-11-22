'use strict';

let Promise = require('bluebird');
let google = require('googleapis');

let gmail = google.gmail('v1');

Promise.promisifyAll(gmail.users.messages);
Promise.promisifyAll(gmail.users.labels);

// Find or create a hidden label by name.
function ensureLabel(auth, name) {
  let list = gmail.users.labels.listAsync({
    auth: auth,
    userId: 'me',
  });

  return list
    .get('labels')
    .filter(l => l.name === name)
    .get(0)
    .then(label => {
      if (label) return label;
      return gmail.users.labels.createAsync({
        auth: auth,
        userId: 'me',
        resource: {
          name: name,
          labelListVisibility: 'labelHide',
          messageListVisibility: 'hide',
        },
      });
    });
}

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
  ensureLabel: ensureLabel,
  listUnread: listUnread,
  getMessage: getMessage,
  archiveMessage: archiveMessage,
};
