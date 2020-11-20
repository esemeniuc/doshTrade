self.addEventListener('push', function (event) {
    const title = 'New message from YoloTrader!';
    const options = {
        body: event.data.text(),
        icon: 'images/icon.png',
        badge: 'images/badge.png'
    };

    const notificationPromise = self.registration.showNotification(title, options);
    event.waitUntil(notificationPromise);
});


self.addEventListener('notificationclick', function (event) {
    console.log('[Service Worker] Notification click Received.');
    event.notification.close();
    event.waitUntil(
        clients.openWindow('https://developers.google.com/web/')
    );
});

