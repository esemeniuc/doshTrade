/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL subscription operation: yoloHandCurated
// ====================================================

export interface yoloHandCurated_stock {
  __typename: "Stock";
  ticker: string;
  price: string;
  percentChange: number;
  rsi: number;
}

export interface yoloHandCurated {
  stock: yoloHandCurated_stock[];
}

export interface yoloHandCuratedVariables {
  tickerSymbols: string[];
}
