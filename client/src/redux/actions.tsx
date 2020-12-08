import {
  PushActionTypes,
  StockSubscriptionActionType,
  PushAction,
  StockSubscriptionAction,
} from "./types";

// push
export const pushPermissionRequest = (): PushAction => ({
  type: PushActionTypes.PERMISSION_REQUESTED,
  payload: { isAsking: true },
});

export const pushPermissionGranted = (): PushAction => ({
  type: PushActionTypes.PERMISSION_GRANTED,
  payload: { userConsent: Notification.permission, isAsking: false },
});

export const pushPermissionDenied = (): PushAction => ({
  type: PushActionTypes.PERMISSION_DENIED,
  payload: { userConsent: Notification.permission, isAsking: false },
});

export const pushPermissionRefreshed = (subscription: PushSubscription): PushAction => ({
  type: PushActionTypes.REGISTRATION_REFRESHED,
  payload: { subscription },
});

// stocks
export const tickerSubscribe = (ticker: string): StockSubscriptionAction => ({
  type: StockSubscriptionActionType.TICKER_ADD,
  payload: { ticker },
});

export const tickerUnsubscribe = (ticker: string): StockSubscriptionAction => ({
  type: StockSubscriptionActionType.TICKER_REMOVE,
  payload: { ticker },
});
