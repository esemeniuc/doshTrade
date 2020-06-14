/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL subscription operation: OversoldStocks
// ====================================================

export interface OversoldStocks_oversoldStocks {
  __typename: "Stock";
  ticker: string;
  price: string;
}

export interface OversoldStocks {
  oversoldStocks: OversoldStocks_oversoldStocks[];
}
