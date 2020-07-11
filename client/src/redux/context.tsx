import React, { createContext, useReducer, Dispatch } from 'react';
import { mainReducer } from './reducers'
import { IState, AppAction, initialState } from './types';
import { useMainEffects } from './effects';

const AppContext = createContext<{
  state: IState;
  dispatch: Dispatch<AppAction>;
}>({
  state: initialState,
  dispatch: () => null
});


const ContextProvider: React.FC = ({ children }) => {
  const [state, dispatch] = useReducer(mainReducer, initialState);
  useMainEffects(state, dispatch)

  return (
    <AppContext.Provider value={{ state, dispatch }}>
      {children}
    </AppContext.Provider>
  )
}

export { ContextProvider, AppContext };