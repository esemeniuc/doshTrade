/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

import { OptionStrategy } from "./globalTypes";

// ====================================================
// GraphQL query operation: getRiskSummary
// ====================================================

export interface getRiskSummary_riskSummary {
  __typename: "OptionRiskSummary";
  maxRisk: string;
  maxProfit: string;
  breakevenAtExpiration: string;
}

export interface getRiskSummary {
  riskSummary: getRiskSummary_riskSummary;
}

export interface getRiskSummaryVariables {
  optionId: string;
  strategy: OptionStrategy;
}
