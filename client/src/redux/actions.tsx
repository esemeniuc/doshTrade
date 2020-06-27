import { PushActionTypes, StockSubscriptionActionType } from "./types";

export const updatePushPermission = () => ({
    type: PushActionTypes.USER_PERMISSION,
    payload: { userConsent: Notification.permission }
})

export const addTicker = (ticker:string) => ({
    type: StockSubscriptionActionType.ADD_TICKER,
    payload: { ticker }
})

export const removeTicker = (ticker:string) => ({
    type: StockSubscriptionActionType.ADD_TICKER,
    payload: { ticker }
})