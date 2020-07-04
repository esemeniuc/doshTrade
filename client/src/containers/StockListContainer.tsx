import React, { useContext } from 'react';
import Container from "@material-ui/core/Container";
import {
    AppBar,
    Box,
    Toolbar, Typography
} from "@material-ui/core";
import { useSubscription } from '@apollo/client';
import { StockPrices_stockPrices } from '../graphql/__generated__/StockPrices'
import { loader } from 'graphql.macro';
import { mockStockData } from "../mocks/mockData";
import StockTableView from "../components/StockTableView";
import TransitionsModal from './TransitionsModal';
import { AppContext } from '../redux/context';

const STOCK_PRICES_SUBSCRIPTION = loader('../graphql/stockPrices.gql');

function StockListContainer() {
    const tickerSymbols = ["TSLA", "BANANA"]
    const { data, loading, error } = useSubscription<StockPrices_stockPrices>(
        STOCK_PRICES_SUBSCRIPTION,
        { variables: { tickerSymbols } }
    );
    const { state: { pushState } } = useContext(AppContext)

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
                    {JSON.stringify(data)}
                </Box>
            </Typography>
            <TransitionsModal open={pushState.isAsking} title={"Push Access"} description={"You will be notified when your favorite stocks dip"} />
            <StockTableView stockData={mockStockData} />
        </Container>
    );
}


export default StockListContainer;