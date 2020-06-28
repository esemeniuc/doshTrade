import React, {ReactElement, useEffect} from 'react';
import {useSubscription} from '@apollo/client';
import {ReversalAlerts} from './graphql/__generated__/ReversalAlerts'
import {loader} from 'graphql.macro';

const REVERSAL_ALERTS_SUBSCRIPTION = loader('./graphql/reversalAlerts.gql');

function handlePermission(permission: NotificationPermission) {
    if(Notification.permission === 'denied' || Notification.permission === 'default') {
        console.log(permission)
    } else {
        console.log(permission)
    }
}

function checkNotificationPromise() {
    try {
        Notification.requestPermission().then();
    } catch(e) {
        return false;
    }
    return true;
}

function askNotificationPermission(){
    if (!('Notification' in window)) {
        console.log("This browser does not support notifications.");
    }else{
        if(checkNotificationPromise()) {
            Notification.requestPermission()
                .then((permission) => {
                    handlePermission(permission);
                })
        } else {
            Notification.requestPermission(function(permission) {
                handlePermission(permission);
            });
        }
    }
}

function NotificationsProvider(props: {children: ReactElement}) {
    useEffect(()=> {
        askNotificationPermission()
    }, [])

    const tickerSymbols = ["TSLA", "BANANA"]
    const { data } = useSubscription<ReversalAlerts>(
        REVERSAL_ALERTS_SUBSCRIPTION,
        { variables: { tickerSymbols } }
    );

    if(data && Notification.permission === 'granted') {
        const img = '/to-do-notifications/img/icon-128.png';
        console.log(data.reversalAlerts)
        data.reversalAlerts.map(stock => {
            const text = `${stock.ticker} is now at price ${stock.price}`;
            return new Notification('Stock reversal!', { body: text, icon: img });
        })
    }

    return props.children;
}


export default NotificationsProvider;