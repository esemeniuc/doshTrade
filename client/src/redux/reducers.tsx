import {
  IPushState,
  PushAction,
  PushActionTypes,
  IState,
  AppAction,
  IStockSubscriptionState,
  StockSubscriptionAction,
  StockSubscriptionActionType,
} from "./types";

export const mainReducer = (
  { pushState, stockSubscriptionState }: IState,
  action: AppAction
) => ({
  pushState: pushReducer(pushState, action as PushAction),
  stockSubscriptionState: stockSubscriptionReducer(
    stockSubscriptionState,
    action as StockSubscriptionAction
  ),
});

const pushReducer = (state: IPushState, action: PushAction): IPushState => {
  switch (action.type) {
    case PushActionTypes.PERMISSION_REQUESTED:
      return {
        ...state,
        isAsking: true,
      };
    case PushActionTypes.PERMISSION_GRANTED:
      return {
        ...state,
        userConsent: action.payload.userConsent,
        isAsking: false,
      };
    case PushActionTypes.PERMISSION_DENIED:
      return {
        ...state,
        userConsent: action.payload.userConsent,
        isAsking: false,
      };
    case PushActionTypes.REGISTRATION_REFRESHED:
      return {
        ...state,
        subscription: action.payload.subscription,
      };
    default:
      return state;
  }
};

const stockSubscriptionReducer = (
  state: IStockSubscriptionState,
  action: StockSubscriptionAction
): IStockSubscriptionState => {
  switch (action.type) {
    case StockSubscriptionActionType.TICKER_ADD:
      return {
        ...state,
        tickers: [...state.tickers, action.payload.ticker],
      };
    case StockSubscriptionActionType.TICKER_REMOVE:
      return {
        ...state,
        tickers:
          state.tickers.length > 0
            ? state.tickers.filter((t) => t !== action.payload.ticker)
            : [],
      };
    default:
      return state;
  }
};
