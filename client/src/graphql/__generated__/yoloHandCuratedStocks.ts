/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL subscription operation: yoloHandCuratedStocks
// ====================================================

export interface yoloHandCuratedStocks_yoloHandCuratedStocks {
  __typename: "Stock";
  ticker: string;
  price: string;
  percentChange: number;
  rsi: number;
}

export interface yoloHandCuratedStocks {
  yoloHandCuratedStocks: yoloHandCuratedStocks_yoloHandCuratedStocks[];
}

export interface yoloHandCuratedStocksVariables {
  tickerSymbols: string[];
}
