import React, { useState } from 'react';
import Container from "@material-ui/core/Container";
import styled from 'styled-components'
import { useQuery } from "@apollo/client";
import { useForm } from "react-hook-form";
import GeneratedResults from '../components/GeneratedResults';
import { loader } from "graphql.macro";
import { getCurrentPrice } from "../graphql/__generated__/getCurrentPrice";
import { getAvailableExpirations } from "../graphql/__generated__/getAvailableExpirations";
import { useDebounce } from "react-use";
import EntryViewStateSpec from "../redux/EntryViewStateSpec"
import { useStateMachine } from '../components/useStateMachine';
import { OptionStrategy, OptionType } from '../graphql/__generated__/globalTypes'
import { ExploreSharp } from '@material-ui/icons';
import { displayStringForExpiryDate } from '../util/dateFormat';

const GET_CURRENT_PRICE_QUERY = loader(
    "../graphql/getCurrentPrice.gql"
);
const GET_AVAILABLE_EXPIRATIONS_QUERY = loader(
    "../graphql/getAvailableExpirations.gql"
);

type StringToStringMap = {
    [key: string]: string;
};

const Title = styled.h2`
  font-size: 2.5em;
  text-align: left;
  color: black;
`;

const GeneratorForm = styled.form`
    font-size: 1em;
`;

// Base class for 
const DoshInput = styled.input`
    display: block;
    width: 100%;
    box-sizing: border-box;
    padding-left: 10px;
    padding-top: 10px;
    padding-bottom: 10px;
    margin-bottom: 5px;
    border-color: gray;
    border-width: 3px;
    border-radius: 7px;
`;
const TickerSearchInput = styled(DoshInput)`
    text-transform: uppercase;
    &::placeholder {
        text-transform: none;
    }
`;

const DoshSelect = styled.select`
    display: block;
    padding-left: 10px;
    padding-top: 10px;
    padding-bottom: 10px;
	width: 100%;
    margin-bottom: 5px;
	box-sizing: border-box;
    border-width: 3px;
    border-radius: 7px;
	-moz-appearance: none;
	-webkit-appearance: none;
	appearance: none;
	background-color: white;
	background-image: url('data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23007CB2%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E');
	background-repeat: no-repeat, repeat;
	background-position: right .7em top 50%, 0 0;
    background-size: .65em auto, 100%;
`;

const ActionButton = styled.button`
    display: block;
    height: 40px;
    box-sizing: border-box;
    padding-left: 10px;
    padding-top: 12px;
    padding-bottom: 12px;
    margin-bottom: 5px;
    border: none;
    border-radius: 7px;
    color: white;
`;

const ActionInput = styled.input`
    display: block;
    height: 40px;
    box-sizing: border-box;
    padding-left: 10px;
    padding-top: 12px;
    padding-bottom: 12px;
    margin-bottom: 5px;
    border: none;
    border-radius: 7px;
    color: white;
`;

const GenerateButton = styled(ActionInput)`
    width: 70%;
    background-color: black;
    position: absolute;
    top: 50%;
    left: 50%;
    margin: 0;
    transform: translate(-50%, -50%);
        &:disabled {
        opacity: 0.3;
    }
`;

const ResetButton = styled(ActionButton)`
    margin-top: 20px;
    width: 100%;
    background-color: black;
`;

const PriceLabel = styled.div`
    text-align: left;
    width: 100%;
    padding-top: 20px;
    padding-left: 10px;
    padding-bottom: 12px;
`
const GeneratedResultsFrame = styled.div`
    background-color: whitesmoke;
    min-height: 300px;
    width: 100%;
    margin-top: 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
`

const EXPIRATION_PLACEHOLDER = "Expiration Date"
const EXPIRATION_LOADING_PLACEHOLDER = "Loading Expiries"
const STRATEGY_PLACEHOLDER = "Strategy"

function displayStringForOptionStrategy(strategy: OptionStrategy) {
    switch (strategy) {
        case OptionStrategy.BUY_CALL:
            return "Buy Call"
        case OptionStrategy.SELL_CALL:
            return "Sell Call"
        case OptionStrategy.BUY_PUT:
            return "Buy Put"
        case OptionStrategy.SELL_PUT:
            return "Sell Put"
    }
}

function optionStrategyFor(strategyInput: String) {
    switch (strategyInput) {
        case "Buy Call":
            return OptionStrategy.BUY_CALL
        case "Sell Call":
            return OptionStrategy.SELL_CALL
        case "Buy Put":
            return OptionStrategy.BUY_PUT
        case "Sell Put":
            return OptionStrategy.SELL_PUT
    }
}

function makeDisplayStringsForExpiryDates(expiries: string[]): StringToStringMap {
    if (!expiries) {
        return {}
    }
    let expiryMap: StringToStringMap = {}
    expiries.forEach((exp, i) => {
        expiryMap[exp] = displayStringForExpiryDate(exp)
    })
    return expiryMap
}

function reverseMap(ogMap: StringToStringMap): StringToStringMap {
    // https://stackoverflow.com/questions/45728226/javascript-map-value-to-keys-reverse-object-mapping
    const reverseMapping = (o: StringToStringMap) => Object.keys(o).reduce((r: StringToStringMap, k: string) =>
        Object.assign(r, { [o[k]]: k }), {})
    return reverseMapping(ogMap)
}

function EntryView() {
    const [debouncedTicker, setDebouncedTicker] = useState('')
    const [currentState, sendEvent] = useStateMachine(EntryViewStateSpec)
    const { register, handleSubmit, watch, reset, errors, trigger } = useForm({
        mode: "onChange"
    });
    const tickerInput = watch(["ticker"]).ticker
    const expirationInput = watch(["expiration"]).expiration
    const strategyInput = watch(["strategy"]).strategy
    const { data: priceData, error: priceError } = useQuery<getCurrentPrice>(GET_CURRENT_PRICE_QUERY, { variables: { ticker: debouncedTicker } });
    const { data: expirationData, error: expirationError, loading: expirationLoading } = useQuery<getAvailableExpirations>(GET_AVAILABLE_EXPIRATIONS_QUERY, { variables: { ticker: debouncedTicker } });
    const onGenerate = (formData: any) => {
        sendEvent("PRESENT_GENERATED_TRADE")
    }
    const priceString = (tickerInput && priceData) ? priceData.price : "$"
    const expirationStrings = (tickerInput && expirationData) ? expirationData.expiration : []
    let expiryDateMap = makeDisplayStringsForExpiryDates(expirationStrings)
    let expiryDateReverseMap = reverseMap(expiryDateMap)
    if (currentState === 'blank' && tickerInput) {
        sendEvent("ENTER_TICKER")
    } else if (currentState === 'enteringTicker' && priceData && !priceError && expirationStrings && !expirationError) {
        sendEvent("TICKER_FETCH_SUCCESS")
    } else if (currentState === 'selectingExpirationAndStrategy' && !tickerInput) {
        sendEvent("ERASE_TICKER")
    }
    useDebounce(() => { tickerInput && setDebouncedTicker(tickerInput) }, 350, [tickerInput])
    const isExpirationAndStrategySelectable =
        currentState === "selectingExpirationAndStrategy" || currentState === "presentingGeneratedTrade"
    const isSubmitButtonEnabled =
        currentState === "selectingExpirationAndStrategy" && expirationInput !== EXPIRATION_PLACEHOLDER && strategyInput !== STRATEGY_PLACEHOLDER
    return (
        <Container component="main" maxWidth="sm" style={{
            backgroundColor: 'white',
            display: 'flex',
            flexDirection: 'column',
            alignContent: 'flex-start',
        }}>
            <Title> Option Analysis </Title>
            <GeneratorForm onSubmit={handleSubmit(onGenerate)}
                onReset={() => sendEvent("RESET")}>
                <TickerSearchInput
                    name="ticker"
                    placeholder="Ticker"
                    ref={register({ required: true })}
                />
                <PriceLabel>
                    Current price: {priceString}
                </PriceLabel>
                <DoshSelect
                    disabled={!isExpirationAndStrategySelectable}
                    name="expiration"
                    defaultValue={
                        expirationLoading ? EXPIRATION_LOADING_PLACEHOLDER : EXPIRATION_PLACEHOLDER
                    } ref={register}
                    onChange={() => {
                        if (currentState === "presentingGeneratedTrade") {
                            sendEvent('SELECT_EXPIRATION_AND_STRATEGY')
                        }
                    }} >
                    <option disabled> {
                        expirationLoading ? EXPIRATION_LOADING_PLACEHOLDER : EXPIRATION_PLACEHOLDER
                    } </option>
                    {expirationStrings.map((exp, i) => <option key={i}>{displayStringForExpiryDate(exp)}</option>)}
                </DoshSelect>
                <DoshSelect
                    disabled={!isExpirationAndStrategySelectable}
                    name="strategy"
                    defaultValue={STRATEGY_PLACEHOLDER}
                    ref={register}
                    onChange={() => {
                        if (currentState === "presentingGeneratedTrade") {
                            sendEvent('SELECT_EXPIRATION_AND_STRATEGY')
                        }
                    }}>
                    <option disabled > Strategy </option>
                    <option>{displayStringForOptionStrategy(OptionStrategy.BUY_CALL)}</option>
                    <option>{displayStringForOptionStrategy(OptionStrategy.SELL_CALL)}</option>
                    <option>{displayStringForOptionStrategy(OptionStrategy.BUY_PUT)}</option>
                    <option>{displayStringForOptionStrategy(OptionStrategy.SELL_PUT)}</option>
                </DoshSelect>
                <GeneratedResultsFrame>
                    {currentState === 'presentingGeneratedTrade' ?
                        <GeneratedResults
                            ticker={debouncedTicker}
                            expiration={expiryDateReverseMap[expirationInput]}
                            strategy={optionStrategyFor(strategyInput)}
                        /> :
                        <GenerateButton
                            disabled={!isSubmitButtonEnabled}
                            type="submit"
                            value="Generate" />
                    }
                </GeneratedResultsFrame>
                {currentState === 'presentingGeneratedTrade' &&
                    <ResetButton
                        onClick={() => reset({ mode: "onChange" })}
                        value="Reset">
                        Reset</ResetButton>
                }
            </GeneratorForm>
        </Container >
    );
}

export default EntryView;
