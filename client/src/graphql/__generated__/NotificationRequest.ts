/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

import { PushSubscription } from "./../../../__generated__/globalTypes";

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
