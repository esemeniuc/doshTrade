/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL subscription operation: ReversalAlerts
// ====================================================

export interface ReversalAlerts_reversalAlerts {
  __typename: "Stock";
  ticker: string;
  price: string;
}

export interface ReversalAlerts {
  reversalAlerts: ReversalAlerts_reversalAlerts[];
}
