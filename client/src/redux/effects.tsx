import React, { useEffect, Dispatch } from 'react';
import { AppAction, IPushState, IState, IStockSubscriptionState, PushAction, StockSubscriptionAction, StockSubscriptionActionType, kPushSubscriptionStorageKey } from './types';
import { askUserPermission, createNotificationSubscription } from '../push/push-notifications';
import { pushPermissionDenied, pushPermissionGranted, subscriptionRegistrationRequest, subscriptionRegistrationSuccess, subscriptionRegistrationFailure } from './actions';
import { setLocalItem } from '../util/localStorage'

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
                    setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, subscription)
                    dispatch(subscriptionRegistrationRequest(subscription));
                    console.log(JSON.stringify(subscription))
                }).catch(err => {
                    console.error("Couldn't create the notification subscription", err, "name:", err.name, "message:", err.message, "code:", err.code);
                });
        } else if (state.userConsent === 'denied') {
            // TODO:
        }
    }, [state.userConsent])

    //TODO: kz looks into refactoring better
    useEffect(() => {
        if (state.isRegistering) {
            const requestOptions = {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(state.subscription)
            };
            fetch('http://localhost:8080/push-subscription', requestOptions)
                .then(response => {
                    if (response.status === 200) {
                        console.log("dispatch subscriptionRegistrationSuccess")
                        dispatch(subscriptionRegistrationSuccess())
                    } else {
                        console.log("dispatch subscriptionRegistrationFailure")
                        dispatch(subscriptionRegistrationFailure())
                    }
                })
        }
    }, [state.isRegistering])

}

const useStockSubscriptionEffects = (state: IStockSubscriptionState, _: Dispatch<StockSubscriptionAction>) => {
    useEffect(() => {
        setLocalItem(StockSubscriptionActionType.TICKER_ADD, state.tickers);
    }, [state.tickers])
}

//TODO: kz takes care of prefix 'use'.
export const useMainEffects = (state: IState, dispatch: Dispatch<AppAction>) => {
    usePushEffects(state.pushState, dispatch)
    useStockSubscriptionEffects(state.stockSubscriptionState, dispatch)
}