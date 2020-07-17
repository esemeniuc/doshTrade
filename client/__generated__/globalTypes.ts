/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

//==============================================================
// START Enums and Input Objects
//==============================================================

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
