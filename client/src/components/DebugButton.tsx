import {Button, Typography} from "@material-ui/core";
import {gql, useQuery} from '@apollo/client';
import React from "react";
import {getLocalItem} from "../util/localStorage";
import {kPushSubscriptionStorageKey} from "../redux/types";

const SEND_DEMO_NOTIFICATION = gql`
    query SendDemoNotification($subscription: PushSubscription!) {
        sendDemoNotification(
            pushSubscription: $subscription
        )
    }
`;

export default function DebugButton() {
    const existingSubscription = getLocalItem<PushSubscription>(
        kPushSubscriptionStorageKey
    );
    const {loading, data, error, refetch} = useQuery(SEND_DEMO_NOTIFICATION, {variables: {subscription: existingSubscription}});
    if (loading) return (<Typography>'Loading Debug...'</Typography>);
    if (error) return (<Typography>`Error Debug! ${error.message}`</Typography>);


    return (<Button onClick={() => refetch()}> DEBUG </Button>)
}
