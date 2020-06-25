import React, {createContext, useReducer} from 'react';
import { pushReducer } from './reducers'
import {  PushStateType, PushActions } from './types';

interface IState {
    pushState: PushStateType
}

const initialState: IState = {
    pushState: {
        userConsent: Notification.permission
    }
}

const AppContext = createContext<{
    state: IState;
    dispatch: React.Dispatch<any>;
  }>({
      state: initialState, 
      dispatch: () => null
    });


const mainReducer = ( state : IState = initialState, action: PushActions) => ({
    pushState: pushReducer(state.pushState, action),
});

const AppProvider: React.FC = ({ children }) => {
  const [state, dispatch] = useReducer(mainReducer, initialState);

  return (
    <AppContext.Provider value={{state, dispatch}}>
      {children}
    </AppContext.Provider>
  )
}

export { AppProvider, AppContext };