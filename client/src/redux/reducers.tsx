import { PushStateType, PushActions, PushActionTypes } from "./types";

export const pushReducer = (state: PushStateType, action: PushActions) => {
    switch (action.type) {
      case PushActionTypes.USER_PERMISSION:
        return {
            ...state,
            userConsent: action.payload,
          }
                    default:
        return state;
    }
}
  

