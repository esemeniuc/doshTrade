export type ActionMap<M extends { [index: string]: any }> = {
  [Key in keyof M]: M[Key] extends undefined
    ? { type: Key }
    : { type: Key; payload: M[Key] };
};

export type AppAction = PushAction | StockSubscriptionAction;

export interface IState {
  pushState: IPushState;
  stockSubscriptionState: IStockSubscriptionState;
}

// push
export enum PushActionTypes {
  PERMISSION_REQUESTED = "PERMISSION_REQUESTED",
  PERMISSION_GRANTED = "PERMISSION_GRANTED",
  PERMISSION_DENIED = "PERMISSION_DENIED",
  REGISTRATION_REQUESTED = "REGISTRATION_REQUESTED",
}

export type IPushState = {
  userConsent: NotificationPermission;
  isAsking: boolean;
  subscription: PushSubscription | null;
};

export type PushPayload = {
  [PushActionTypes.PERMISSION_REQUESTED]: {
    isAsking: boolean;
  };
  [PushActionTypes.PERMISSION_GRANTED]: {
    isAsking: boolean;
    userConsent: NotificationPermission;
  };
  [PushActionTypes.PERMISSION_DENIED]: {
    isAsking: boolean;
    userConsent: NotificationPermission;
  };
  [PushActionTypes.REGISTRATION_REQUESTED]: {
    subscription: PushSubscription;
  };
};

export type PushAction = ActionMap<PushPayload>[keyof ActionMap<PushPayload>];

// stocks
export enum StockSubscriptionActionType {
  TICKER_ADD = "TICKER_ADD",
  TICKER_REMOVE = "TICKER_REMOVE",
}

export type IStockSubscriptionState = {
  tickers: string[];
};

export type StockSubscriptionPayload = {
  [StockSubscriptionActionType.TICKER_ADD]: {
    ticker: string;
  };
  [StockSubscriptionActionType.TICKER_REMOVE]: {
    ticker: string;
  };
};

export type StockSubscriptionAction = ActionMap<
  StockSubscriptionPayload
>[keyof ActionMap<StockSubscriptionPayload>];
