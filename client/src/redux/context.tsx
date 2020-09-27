import React, {createContext, Dispatch, ReactNode, useReducer} from "react";
import {mainReducer} from "./reducers";
import {AppAction, initialState, IState} from "./types";
import {useMainEffects} from "./effects";

const AppContext = createContext<{
  state: IState;
  dispatch: Dispatch<AppAction>;
}>({
  state: initialState,
  dispatch: () => null,
});

function ContextProvider(props: { children: ReactNode }) {
  const [state, dispatch] = useReducer(mainReducer, initialState);
  useMainEffects(state, dispatch);

  return (
      <AppContext.Provider value={{state, dispatch}}>
        {props.children}
      </AppContext.Provider>
  );
}

export { ContextProvider, AppContext };
