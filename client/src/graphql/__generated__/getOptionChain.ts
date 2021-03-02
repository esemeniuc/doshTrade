/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

import { OptionStrategy, OptionType } from "./globalTypes";

// ====================================================
// GraphQL query operation: getOptionChain
// ====================================================

export interface getOptionChain_optionQuote {
  __typename: "OptionQuote";
  optionType: OptionType;
  stringId: string;
  strike: number | null;
  expiration: string;
  bid: number | null;
  ask: number | null;
  last: number | null;
  delta: number;
  gamma: number;
  theta: number;
  vega: number;
  rho: number;
  volatility: number;
  timeValue: number;
}

export interface getOptionChain {
  optionQuote: getOptionChain_optionQuote[];
}

export interface getOptionChainVariables {
  ticker: string;
  expiration: string;
  strategy: OptionStrategy;
}
