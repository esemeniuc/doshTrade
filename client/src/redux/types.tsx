export type ActionMap<M extends { [index: string]: any }> = {
    [Key in keyof M]: M[Key] extends undefined
      ? {
          type: Key;
        }
      : {
          type: Key;
          payload: M[Key];
        }
  };

export type AppAction = PushAction | StockSubscriptionAction

export interface IState {
    pushState: IPushState,
    stockSubscriptionState: IStockSubscriptionState
}

// push
export enum PushActionTypes {
    USER_PERMISSION = 'USER_PERMISSION',
}  

export type IPushState = {
    userConsent: NotificationPermission
}

export type PushPayload = {
    [PushActionTypes.USER_PERMISSION]: {
        userConsent: NotificationPermission;
    }
}

export type PushAction = ActionMap<PushPayload>[keyof ActionMap<PushPayload>];

// stocks
export enum StockSubscriptionActionType {
    ADD_TICKER = "ADD_TICKER",
    REMOVE_TICKER = "REMOVE_TICKER"
}

export type IStockSubscriptionState = {
    tickers: string[]
}

export type StockSubscriptionPayload = {
    [StockSubscriptionActionType.ADD_TICKER]: {
        ticker: string
    },
    [StockSubscriptionActionType.REMOVE_TICKER]: {
        ticker: string
    },
}

export type StockSubscriptionAction = ActionMap<StockSubscriptionPayload>[keyof ActionMap<StockSubscriptionPayload>];