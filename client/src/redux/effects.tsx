import { useEffect, Dispatch } from 'react';
import { AppAction, IPushState, IState, IStockSubscriptionState, PushAction, StockSubscriptionAction, StockSubscriptionActionType, kPushSubscriptionStorageKey } from './types';
import { createNotificationSubscription } from '../push/push-notifications';
import { pushPermissionDenied, pushPermissionGranted } from './actions';
import { setLocalItem, getLocalItem } from '../util/localStorage'
import { useMutation } from '@apollo/client';
import { loader } from 'graphql.macro';

const PUSH_NOTIFICATION_SUBSCRIPTION = loader('../graphql/notificationRequest.gql');


const usePushEffects = (state: IPushState, dispatch: Dispatch<PushAction>) => {
    const [addPushSubscription] = useMutation(PUSH_NOTIFICATION_SUBSCRIPTION);
    useEffect(() => {
        if (state.isAsking) {
            setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, null)
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
            const existingSubscription = getLocalItem<PushSubscription>(kPushSubscriptionStorageKey)
            if (existingSubscription) {
                console.log(JSON.stringify(existingSubscription))
                return
            }
            createNotificationSubscription()
                .then(function (subscription) {
                    setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, subscription)
                    addPushSubscription({ variables: { tickerSymbols: [], subscription } })
                    console.log("Fresh subscription! \n", JSON.stringify(subscription))
                }).catch(err => {
                    console.error("Couldn't create the notification subscription", err, "name:", err.name, "message:", err.message, "code:", err.code);
                });
        } else if (state.userConsent === 'denied') {
            // TODO:
        }
    }, [state.userConsent])
}

const useStockSubscriptionEffects = (state: IStockSubscriptionState, _: Dispatch<StockSubscriptionAction>) => {
    const [addPushSubscription] = useMutation(PUSH_NOTIFICATION_SUBSCRIPTION);
    useEffect(() => {
        setLocalItem(StockSubscriptionActionType.TICKER_ADD, state.tickers);
        const subscription = getLocalItem<PushSubscription>(kPushSubscriptionStorageKey)
        if (!subscription) {
            console.log("Ticker added without a subscription!!")
            return
        }
        addPushSubscription({ variables: { tickerSymbols: [], subscription } })
    }, [state.tickers])
}

//TODO: kz takes care of prefix 'use'.
export const useMainEffects = (state: IState, dispatch: Dispatch<AppAction>) => {
    usePushEffects(state.pushState, dispatch)
    useStockSubscriptionEffects(state.stockSubscriptionState, dispatch)
}