import React, {useEffect} from 'react';
import {
    Box,
    Typography
} from "@material-ui/core";
import {useSubscription} from '@apollo/client';
import {ReversalAlerts_reversalAlerts} from './graphql/__generated__/ReversalAlerts'
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

function NotificationsProvider() {
    useEffect(()=>{
        askNotificationPermission()
        if(Notification.permission === 'granted') {
            const img = '/to-do-notifications/img/icon-128.png';
            const text = 'HEY! Your task is now overdue.';
            setTimeout(()=>{
                const notification = new Notification('To do list', { body: text, icon: img });
            }, 3000)
        }
    }, [])


    const tickerSymbols = ["TSLA", "BANANA"]
    const { data, loading, error } = useSubscription<ReversalAlerts_reversalAlerts>(
        REVERSAL_ALERTS_SUBSCRIPTION,
        { variables: { tickerSymbols } }
    );

    if (loading) return <Typography variant="caption">
        <Box textAlign="center">
            loading ...
        </Box>
    </Typography>

    if (error) return <Typography variant="caption">
        <Box textAlign="center">
            error!!
        </Box>
    </Typography>

    return (
        <div>
            {JSON.stringify(data)}
        </div>
    );
}


export default NotificationsProvider;