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
import { createNotificationSubscription } from "../push/push-notifications";
import { pushPermissionDenied, pushPermissionGranted } from "./actions";
import { setLocalItem, getLocalItem } from "../util/localStorage";

const PUSH_NOTIFICATION_SUBSCRIPTION = loader(
  "../graphql/notificationRequest.gql"
);

const usePushEffects = (state: IPushState, dispatch: Dispatch<PushAction>) => {
  const [addPushSubscription] = useMutation(PUSH_NOTIFICATION_SUBSCRIPTION);
  useEffect(() => {
    if (state.isAsking) {
      setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, null);
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
    if (state.userConsent === "granted") {
      const existingSubscription = getLocalItem<PushSubscription>(
        kPushSubscriptionStorageKey
      );
      if (existingSubscription) {
        console.log(JSON.stringify(existingSubscription));
        return;
      }
      createNotificationSubscription()
        .then(function (pushSubscription) {
          setLocalItem<PushSubscription>(kPushSubscriptionStorageKey, pushSubscription);
          addPushSubscription({
            variables: { tickerSymbols: [], subscription: pushSubscription }, //reset server state if server has old subscription
          });
          console.log("Fresh subscription! \n", JSON.stringify(pushSubscription));
        })
        .catch((err) => {
          console.error(
            "Couldn't create the notification subscription",
            err,
            "name:",
            err.name,
            "message:",
            err.message,
            "code:",
            err.code
          );
        });
    } else if (state.userConsent === "denied") {
      // TODO:
    }
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
