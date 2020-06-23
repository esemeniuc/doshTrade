/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL subscription operation: StockPrices
// ====================================================

export interface StockPrices_stockPrices {
  __typename: "Stock";
  ticker: string;
  price: string;
}

export interface StockPrices {
  stockPrices: StockPrices_stockPrices[];
}

export interface StockPricesVariables {
  tickerSymbols: string[];
}
