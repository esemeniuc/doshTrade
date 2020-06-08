/* tslint:disable */
/* eslint-disable */
// @generated
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL mutation operation: CreatePropertyMutation
// ====================================================

export interface CreatePropertyMutation_createProperty {
  __typename: "Property";
  id: string;
}

export interface CreatePropertyMutation {
  createProperty: CreatePropertyMutation_createProperty | null;
}

export interface CreatePropertyMutationVariables {
  websiteName: string;
  websiteUrl: string;
}
