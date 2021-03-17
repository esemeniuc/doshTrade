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
  delta: number | null;
  gamma: number | null;
  theta: number | null;
  vega: number;
  rho: number | null;
  volatility: number | null;
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
