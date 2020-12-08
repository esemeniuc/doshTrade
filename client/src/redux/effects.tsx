import { useEffect, Dispatch } from "react";
import { useMutation } from "@apollo/client";
import { loader } from "graphql.macro";
import {
  AppAction,
  IPushState,
  IState,
  IStockSubscriptionState,
  PushAction,
  StockSubscriptionAction,
  StockSubscriptionActionType,
  kPushSubscriptionStorageKey,
} from "./types";
import { createNotificationSubscription, getUserSubscription } from "../push/push-notifications";
import { pushPermissionDenied, pushPermissionGranted, pushPermissionRefreshed } from "./actions";
import { setLocalItem, getLocalItem } from "../util/localStorage";

const PUSH_NOTIFICATION_SUBSCRIPTION = loader(
  "../graphql/notificationRequest.gql"
);

const usePushEffects = (state: IPushState, dispatch: Dispatch<PushAction>) => {
  const [addPushSubscription] = useMutation(PUSH_NOTIFICATION_SUBSCRIPTION);
  useEffect(() => {
    if (state.isAsking) {
      Notification.requestPermission().then((consent) => {
        if (consent !== "granted") {
          dispatch(pushPermissionDenied());
        } else {
          dispatch(pushPermissionGranted());
        }
      });
    }
  }, [state.isAsking]);

  useEffect(() => {
    if (state.subscription) {
      setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, state.subscription);
    } else {
      setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, null);
    }
  }, [state.subscription])

  useEffect(() => {
    if (state.userConsent !== "granted") {
      console.log("userConsent not granted! state: ", state.userConsent)
      return
    }
    // A granted status could either mean that they recently granted it or from a previous session
    // so check for existing subscription before creating a new one.
    getUserSubscription().then((pushSubscription) => {
      if (pushSubscription) {
        dispatch(pushPermissionRefreshed(pushSubscription))
      } else {
        createNotificationSubscription()
          .then(function (pushSubscription) {
            if (pushSubscription != null) {
              addPushSubscription({
                variables: { tickerSymbols: [], pushSubscription }, //reset server state if server has old subscription
              });
              console.log("Fresh subscription! \n", JSON.stringify(pushSubscription));
              dispatch(pushPermissionRefreshed(pushSubscription))
            }
          })
          .catch((err) => {
            console.error(
              "Couldn't create the notification subscription", err,
              "name:", err.name,
              "message:", err.message,
              "code:", err.code
            )
          })
      }
    })
  }, [state.userConsent]);
};

const useStockSubscriptionEffects = (
  state: IStockSubscriptionState,
  _: Dispatch<StockSubscriptionAction>
) => {
  const [addPushSubscription] = useMutation(PUSH_NOTIFICATION_SUBSCRIPTION);
  useEffect(() => {
    setLocalItem(StockSubscriptionActionType.TICKER_ADD, state.tickers);
    const pushSubscription = getLocalItem<PushSubscription>(
      kPushSubscriptionStorageKey
    );
    if (!pushSubscription) {
      console.log("Ticker added without a subscription!!");
      return;
    }
    addPushSubscription({ variables: { tickerSymbols: state.tickers, pushSubscription } });
  }, [state.tickers]);
};

// TODO: kz takes care of prefix 'use'.
export const useMainEffects = (
  state: IState,
  dispatch: Dispatch<AppAction>
) => {
  usePushEffects(state.pushState, dispatch);
  useStockSubscriptionEffects(state.stockSubscriptionState, dispatch);
};
