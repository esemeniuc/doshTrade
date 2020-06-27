import React, {createContext, useReducer, Dispatch} from 'react';
import { mainReducer } from './reducers'
import { IState, AppAction } from './types';


const initialState: IState = {
    pushState: { userConsent: Notification.permission },
    stockSubscriptionState: { tickers: [] }
}

const AppContext = createContext<{
    state: IState;
    dispatch: Dispatch<AppAction>;
  }>({
      state: initialState, 
      dispatch: () => null
});

const ContextProvider: React.FC = ({ children }) => {
  const [state, dispatch] = useReducer(mainReducer, initialState);
  return (
    <AppContext.Provider value={{state, dispatch}}>
      {children}
    </AppContext.Provider>
  )
}

export { ContextProvider, AppContext };