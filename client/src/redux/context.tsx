import React, { createContext, useReducer, Dispatch } from "react";
import mainReducer from "./reducers";
import { IState, AppAction, StockSubscriptionActionType } from "./types";
import { useMainEffects } from "./effects";
import { getLocalItem } from "../util/localStorage";
import { kPushSubscriptionStorageKey } from "../constants";

const initialPushState = {
  userConsent: Notification.permission,
  isAsking: false,
  subscription: getLocalItem<PushSubscription>(kPushSubscriptionStorageKey),
};

const initialStockSubscriptionState = {
  tickers: getLocalItem<string[]>(StockSubscriptionActionType.TICKER_ADD) ?? [],
};

export const initialState: IState = {
  pushState: initialPushState,
  stockSubscriptionState: initialStockSubscriptionState,
};

const AppContext = createContext<{
  state: IState;
  dispatch: Dispatch<AppAction>;
}>({
  state: initialState,
  dispatch: () => null,
});

const ContextProvider: React.FC = ({ children }) => {
  const [state, dispatch] = useReducer(mainReducer, initialState);
  useMainEffects(state, dispatch);

  return (
    <AppContext.Provider value={{ state, dispatch }}>
      {children}
    </AppContext.Provider>
  );
};

export { ContextProvider, AppContext };
