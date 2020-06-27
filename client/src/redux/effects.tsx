import React, { useEffect, Dispatch } from 'react';
import { AppAction, IPushState, IState, IStockSubscriptionState, PushAction, StockSubscriptionAction, StockSubscriptionActionType } from './types';
import { askUserPermission, createNotificationSubscription } from '../push/push-notifications';
import { pushPermissionDenied, pushPermissionGranted, subscriptionRegistrationRequest, subscriptionRegistrationSuccess } from './actions';

const usePushEffects = (state: IPushState, dispatch: Dispatch<PushAction>) => {
    useEffect(() => {
        if (state.isAsking) {
            Notification.requestPermission().then(consent => {
                if (consent !== "granted") {
                    dispatch(pushPermissionDenied())
                } else {
                    dispatch(pushPermissionGranted())
                }
            })
        }
    }, [state.isAsking])

    useEffect(() => {
        if (state.userConsent === 'granted') {
            createNotificationSubscription()
                .then(function (subscription) {
                    dispatch(subscriptionRegistrationRequest(subscription));
                }).catch(err => {
                    console.error("Couldn't create the notification subscription", err, "name:", err.name, "message:", err.message, "code:", err.code);
                });
        } else if (state.userConsent === 'denied') {
            // TODO:
        }
        console.log("usePushEffects  userConsent: ", state.userConsent)
    }, [state.userConsent])

    //TODO: kz looks into refactoring better
    useEffect(() => {
        if (state.isRegistering) {
            // TODO: send subscription to server
            // then
            dispatch(subscriptionRegistrationSuccess())
        } else {
            // done registering, good to go!
        }
        console.log("usePushEffects  isRegistering: ", state.isRegistering)
    }, [state.isRegistering])

}

const useStockSubscriptionEffects = (state: IStockSubscriptionState, dispatch: Dispatch<StockSubscriptionAction>) => {
    useEffect(() => {
        window.localStorage.setItem(StockSubscriptionActionType.TICKER_ADD, JSON.stringify(state.tickers));
    }, [state.tickers])
}

//TODO: kz takes care of prefix 'use'.
export const useMainEffects = (state: IState, dispatch: Dispatch<AppAction>) => {
    usePushEffects(state.pushState, dispatch)
    useStockSubscriptionEffects(state.stockSubscriptionState, dispatch)
}