/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

//==============================================================
// START Enums and Input Objects
//==============================================================

export enum OptionStrategy {
  BUY_CALL = "BUY_CALL",
  BUY_PUT = "BUY_PUT",
  SELL_CALL = "SELL_CALL",
  SELL_PUT = "SELL_PUT",
}

export enum OptionType {
  CALL = "CALL",
  PUT = "PUT",
}

export interface PushSubscription {
  endpoint: string;
  expirationTime?: string | null;
  keys: PushSubscriptionKeys;
}

export interface PushSubscriptionKeys {
  p256dh: string;
  auth: string;
}

//==============================================================
// END Enums and Input Objects
//==============================================================
