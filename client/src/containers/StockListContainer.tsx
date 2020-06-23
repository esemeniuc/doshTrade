import React from 'react';
import Container from "@material-ui/core/Container";
import {
    AppBar,
    Box,
    Toolbar, Typography
} from "@material-ui/core";
import {useSubscription} from '@apollo/client';
import {OversoldStocks} from '../graphql/__generated__/OversoldStocks'
import {loader} from 'graphql.macro';
import {mockStockData} from "../mocks/mockData";
import StockTableView from "../components/StockTableView";

const OVERSOLD_STOCK_SUBSCRIPTION = loader('../graphql/oversoldStocks.gql');

function StockListContainer() {
    const { data, loading, error } = useSubscription<OversoldStocks>(
        OVERSOLD_STOCK_SUBSCRIPTION
    );

    if (loading) return <Typography variant="caption">
        <Box textAlign="center">
            loading ...
        </Box>
    </Typography>

    if (error) return <Typography variant="caption">
        <Box textAlign="center">
            error!!
        </Box>
    </Typography>

    return (
        <>
            <AppBar position="static">
                <Toolbar>
                    <Typography variant="h6">
                        Yolo Trader
                    </Typography>
                </Toolbar>
            </AppBar>
            <Container component="main" maxWidth='sm'>
                <Typography variant="caption">
                    <Box textAlign="center">
                        Since close yesterday
                    </Box>
                </Typography>
                <Typography variant="caption">
                    <Box textAlign="center">
                        {JSON.stringify(data)}
                    </Box>
                </Typography>
                <StockTableView stockData={mockStockData}/>
            </Container>
        </>
    );
}


export default StockListContainer;