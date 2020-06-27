import { IPushState, PushAction, PushActionTypes, IState, AppAction, IStockSubscriptionState, StockSubscriptionAction, StockSubscriptionActionType } from "./types";


export const mainReducer = ( { pushState, stockSubscriptionState } : IState, action: AppAction) => ({
    pushState: pushReducer(pushState, action as PushAction),
    stockSubscriptionState: stockSubscriptionReducer(stockSubscriptionState, action as StockSubscriptionAction)
});

const pushReducer = (state: IPushState, action: PushAction): IPushState => {
    switch (action.type) {
      case PushActionTypes.USER_PERMISSION:
        return {
            ...state,
            userConsent: action.payload.userConsent,
          }
        default:
            return state;
    }
}
  
const stockSubscriptionReducer = (state: IStockSubscriptionState, action: StockSubscriptionAction): IStockSubscriptionState => {
    switch (action.type) {
        case StockSubscriptionActionType.ADD_TICKER:
          return {
              ...state,
            }
        case StockSubscriptionActionType.REMOVE_TICKER:
            return {
                ...state,
                }
        default:
            return state;
      }
  }