import { Button, Typography } from "@material-ui/core";
import { gql, useQuery } from '@apollo/client';
import React from "react";

const GET_DEBUG = gql`
  query GetDebug($subscription: PushSubscription!) {
    getDebug(
        pushSubscription: $subscription
    )
  }
`;

export default function DebugButton(props: {}) {
    const { loading, data, error, refetch } = useQuery(GET_DEBUG, {variables: {subscription: {endpoint: "endpoint!!!", keys: { p256dh: "peepee", auth: "tokenpoken"}}}});
    if (loading) return (<Typography>'Loading Debug...'</Typography>);
    if (error) return (<Typography>`Error Debug! ${error.message}`</Typography>);

    return (<Button onClick={() => refetch()}> DEBUG </Button>)
}