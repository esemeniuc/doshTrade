import React from 'react';
import styled from 'styled-components'
import { getOptionChain, getOptionChain_optionQuote as OptionQuote } from '../graphql/__generated__/getOptionChain';
import OptionTable from './OptionTable'
import { useQuery } from "@apollo/client";
import { loader } from 'graphql.macro';
import { OptionalEventProperties } from 'react-dom/test-utils';

const GET_OPTION_CHAIN = loader(
    "../graphql/getOptionChain.gql"
);


const GeneratedOption = styled.div`
    width: 100%;
    margin-top: 20px;
`
const RiskSummaryTable = styled.table`
    width: 100%;
    margin-top: 20px;
    margin-left: 10px;
    margin-right: 10px;
    border-spacing: 0px;
`
const RiskRow = styled.tr`
    height: 40px;
`
const RiskColumnLeft = styled.td`
    border-right: 1px solid black;
    text-align: left;
    width: 40%;
    font-weight: 600;
    font-size: 12px;
`
const RiskColumnRight = styled.td`
    padding-left: 20px;
    text-align: left;
    font-size: 12px;
`


const RiskSummary = (props: any) => {
    return (
        <RiskSummaryTable>
            <tbody>
                <RiskRow>
                    <RiskColumnLeft>Maximum Risk</RiskColumnLeft>
                    <RiskColumnRight>$1000</RiskColumnRight>
                </RiskRow>
                <RiskRow>
                    <RiskColumnLeft>Maximum Profit</RiskColumnLeft>
                    <RiskColumnRight>$350</RiskColumnRight>
                </RiskRow>
                <RiskRow>
                    <RiskColumnLeft>Break even at expiry</RiskColumnLeft>
                    <RiskColumnRight>$232.35</RiskColumnRight>
                </RiskRow>
            </tbody>
        </RiskSummaryTable>
    )
}

const handleSelectOption = (optionQuote: OptionQuote) => {
    console.log("handleSelectOption")
}

export default function GeneratedResults({ ticker, expiration, strategy }: any) {
    const { data } = useQuery<getOptionChain>(GET_OPTION_CHAIN, { variables: { ticker, expiration, strategy } });
    //TODO: optionQuote should be the default option to display
    const optionQuote = data ? data.optionQuote : []
    return (
        <GeneratedOption>
            <OptionTable optionQuotes={optionQuote} onSelectOption={handleSelectOption} />
            <RiskSummary />
        </GeneratedOption>
    )
}
