/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: PrivateModeStatsByDateQuery
// ====================================================

export interface PrivateModeStatsByDateQuery_privateModeStatsByDate {
  __typename: "PrivateCounts";
  date: string;
  privateCount: number;
  nonPrivateCount: number;
}

export interface PrivateModeStatsByDateQuery {
  privateModeStatsByDate: PrivateModeStatsByDateQuery_privateModeStatsByDate[] | null;
}

export interface PrivateModeStatsByDateQueryVariables {
  propertyId: string;
  startDate: string;
  endDate: string;
}
