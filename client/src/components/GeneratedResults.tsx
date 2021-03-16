import React, { useState } from 'react';
import styled from 'styled-components'
import { getOptionChain, getOptionChain_optionQuote as OptionQuote } from '../graphql/__generated__/getOptionChain';
import { getRiskSummary } from '../graphql/__generated__/getRiskSummary';
import OptionTable from './OptionTable'
import { useQuery } from "@apollo/client";
import { loader } from 'graphql.macro';
import TransitionsModal from "../containers/TransitionsModal";
import { OptionStrategy, OptionType } from '../graphql/__generated__/globalTypes'

const GET_OPTION_CHAIN = loader(
    "../graphql/getOptionChain.gql"
);

const GET_RISK_SUMMARY = loader("../graphql/getRiskSummary.gql")

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
const SelectorModalHeader = styled.h3`
    margin-top: 10px;
    margin-bottom: 10px;
    width: 100%;
`

// mocking..
const mockOptions: OptionQuote[] = [{
    __typename: 'OptionQuote',
    stringId: "option1",
    optionType: OptionType.CALL,
    expiration: "3/15",
    ask: 13.2,
    delta: 1,
    gamma: 1,
    theta: 1,
    vega: 1,
    rho: 1,
    volatility: 1,
    timeValue: 1,
    strike: 1, bid: 1, last: 1
},
{
    __typename: 'OptionQuote',
    stringId: "option2",
    optionType: OptionType.CALL,
    expiration: "3/15",
    ask: 113.2,
    delta: 1,
    gamma: 1,
    theta: 1,
    vega: 1,
    rho: 1,
    volatility: 1,
    timeValue: 1,
    strike: 1, bid: 1, last: 1
},
{
    __typename: 'OptionQuote',
    stringId: "option3",
    optionType: OptionType.CALL,
    expiration: "3/15",
    ask: 213.2,
    delta: 1,
    gamma: 1,
    theta: 1,
    vega: 1,
    rho: 1,
    volatility: 1,
    timeValue: 1,
    strike: 1, bid: 1, last: 1
}]

const RiskSummary = ({ selectedOption, strategy }:
    {
        selectedOption?: OptionQuote,
        strategy: OptionStrategy
    }) => {
    const { data } = useQuery<getRiskSummary>(GET_RISK_SUMMARY,
        { variables: { optionId: selectedOption ? selectedOption.bid : 1, strategy } });
    if (!data) {
        return (null)
    }
    const { maxRisk, maxProfit, breakevenAtExpiration } = data.riskSummary
    return (
        <RiskSummaryTable>
            <tbody>
                <RiskRow>
                    <RiskColumnLeft>Maximum Risk</RiskColumnLeft>
                    <RiskColumnRight>{maxRisk}</RiskColumnRight>
                </RiskRow>
                <RiskRow>
                    <RiskColumnLeft>Maximum Profit</RiskColumnLeft>
                    <RiskColumnRight>{maxProfit}</RiskColumnRight>
                </RiskRow>
                <RiskRow>
                    <RiskColumnLeft>Break even at expiry</RiskColumnLeft>
                    <RiskColumnRight>{breakevenAtExpiration}</RiskColumnRight>
                </RiskRow>
            </tbody>
        </RiskSummaryTable>
    )
}

export default function GeneratedResults({ ticker, expiration, strategy }: any) {
    const { data } = useQuery<getOptionChain>(GET_OPTION_CHAIN, { variables: { ticker, expiration, strategy } });
    const [selectedOption, setSelectedOption] = useState<OptionQuote | undefined>()
    const [modalOpen, setModalOpen] = useState(false)
    let optionQuotes: OptionQuote[] = []
    if (data && data.optionQuote.length > 0 && !selectedOption) {
        optionQuotes = data ? data.optionQuote : []
        console.log('set optionQuote:', optionQuotes)
        setSelectedOption(data.optionQuote[0])
    }
    const handleSelectOption = (optionQuote: OptionQuote) => {
        console.log("selected option: ", optionQuote)
        setSelectedOption(optionQuote)
        // TODO: dispatch modal action with all the options
        setModalOpen(!modalOpen)
    }
    return (
        <GeneratedOption>
            <OptionTable
                optionQuotes={selectedOption ? [selectedOption] : []}
                selectedOption={selectedOption}
                onSelectOption={handleSelectOption} />
            <RiskSummary
                selectedOption={selectedOption}
                strategy={strategy} />
            <TransitionsModal
                open={modalOpen}
                onClose={() => setModalOpen(false)}
            >
                <SelectorModalHeader>
                    {strategy}
                    <br />
                    {ticker} {expiration}
                </SelectorModalHeader>
                {/* <OptionTable optionQuotes={data ? data.optionQuote : []} onSelectOption={handleSelectOption} /> */}
                <OptionTable
                    optionQuotes={data && data.optionQuote ? data.optionQuote : []}
                    selectedOption={selectedOption}
                    onSelectOption={handleSelectOption} />
            </TransitionsModal>
        </GeneratedOption>
    )
}
