/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: PrivateModeStatsQuery
// ====================================================

export interface PrivateModeStatsQuery_privateModeStats {
  __typename: "PrivateCounts";
  date: string;
  privateCount: number;
  nonPrivateCount: number;
}

export interface PrivateModeStatsQuery {
  privateModeStats: PrivateModeStatsQuery_privateModeStats[] | null;
}

export interface PrivateModeStatsQueryVariables {
  propertyId: string;
}
