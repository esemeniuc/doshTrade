/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

import { PushSubscription } from "./globalTypes";

// ====================================================
// GraphQL mutation operation: NotificationRequest
// ====================================================

export interface NotificationRequest {
  notificationRequest: string[];
}

export interface NotificationRequestVariables {
  tickerSymbols: string[];
  pushSubscription: PushSubscription;
}
