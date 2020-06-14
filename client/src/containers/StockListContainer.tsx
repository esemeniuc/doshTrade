import React from 'react';

import Container from "@material-ui/core/Container";
import {
    AppBar,
    Box,
    Toolbar, Typography
} from "@material-ui/core";
import {ApolloError, useSubscription} from '@apollo/client';
import {loader} from 'graphql.macro';

const STOCK_PRICES_SUBSCRIPTION = loader('../graphql/stockPrices.gql');


function StockListContainer() {
    const tickerSymbols = ["AAPL", "BANANA"]
    const { data, loading, error } = useSubscription(
        STOCK_PRICES_SUBSCRIPTION,
        { variables: { tickerSymbols } }
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
        <Container component="main" maxWidth='sm'>
            <AppBar position="static">
                <Toolbar>
                    <Typography variant="h6">
                        Yolo Trader
                    </Typography>
                </Toolbar>
            </AppBar>
            <Typography variant="caption">
                <Box textAlign="center">
                    Since close yesterday
                </Box>
            </Typography>
            <Typography variant="caption">
                <Box textAlign="center">
                    {data}
                </Box>
            </Typography>
        </Container>
    );
}


export default StockListContainer;