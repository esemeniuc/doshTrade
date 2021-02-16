import React, { useState } from 'react';
import Container from "@material-ui/core/Container";
import styled from 'styled-components'
import { useQuery } from "@apollo/client";
import { useForm } from "react-hook-form";
import GeneratedResults from '../components/GeneratedResults';
import { loader } from "graphql.macro";
import { getCurrentPrice } from "../graphql/__generated__/getCurrentPrice";
import { useDebounce } from "react-use";

const GET_CURRENT_PRICE_QUERY = loader(
    "../graphql/getCurrentPrice.gql"
);

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

const ActionButton = styled.input`
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

const GenerateButton = styled(ActionButton)`
    width: 70%;
    background-color: black;
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
    background-color: gainsboro;
    min-height: 300px;
    width: 100%;
    margin-top: 20px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
`

function EntryView() {
    const [submitted, setSubmitted] = useState(false)
    const [debouncedTicker, setDebouncedTicker] = useState('')
    const onGenerate = (formData: any) => {
        console.log("onGenerate called")
        setSubmitted(true)
    }
    const onAfterReset = (e: any) => {
        setSubmitted(false)
    }

    const { register, handleSubmit, watch, errors, trigger } = useForm({
        mode: "onChange"
    });
    const ticker = watch(["ticker"]).ticker
    const { data, loading, error } = useQuery<getCurrentPrice>(GET_CURRENT_PRICE_QUERY, { variables: { ticker: debouncedTicker } });
    const priceString = data ? data.price : "$"

    useDebounce(
        () => {
            ticker && setDebouncedTicker(ticker)
            console.log('ticker: ', ticker)
        },
        350,
        [ticker]
    );

    return (
        <Container component="main" maxWidth="sm" style={{
            backgroundColor: 'white',
            display: 'flex',
            flexDirection: 'column',
            alignContent: 'flex-start',
        }}>
            <Title> Option Analysis </Title>
            <GeneratorForm onSubmit={handleSubmit(onGenerate)}
                onReset={onAfterReset}>
                <TickerSearchInput
                    name="ticker"
                    placeholder="Ticker"
                    ref={register({ required: true })}
                />
                <PriceLabel>
                    Current price: {priceString}
                </PriceLabel>
                <DoshSelect name="expiration" defaultValue="Expiration Date" ref={register} >
                    <option disabled> Expiration Date </option>
                    <option>Apples</option>
                    <option>Pears</option>
                </DoshSelect>
                <DoshSelect name="strategy" defaultValue="Strategy" ref={register} >
                    <option disabled > Strategy </option>
                    <option>Buy Call</option>
                    <option>Sell Call</option>
                    <option>Buy Put</option>
                    <option>Sell Put</option>
                </DoshSelect>
                <GeneratedResultsFrame>
                    {submitted ?
                        <GeneratedResults /> :
                        <GenerateButton type="submit" value="Submit" />
                    }
                </GeneratedResultsFrame>
                {submitted &&
                    <ResetButton type="reset" value="Reset" />
                }
            </GeneratorForm>
        </Container >
    );
}

export default EntryView;
