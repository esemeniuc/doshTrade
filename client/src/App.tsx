import React, { useEffect } from 'react';
import { BrowserRouter as Router } from "react-router-dom";
import { ApolloClient, ApolloProvider, HttpLink, InMemoryCache, split } from '@apollo/client';
import { createMuiTheme, CssBaseline, ThemeProvider } from "@material-ui/core";
import { green, red } from "@material-ui/core/colors";
import { getMainDefinition } from '@apollo/client/utilities';
import { WebSocketLink } from '@apollo/link-ws';
import StockListContainer from "./containers/StockListContainer";
import * as registerServiceWorker from './push/registerServiceWorker';
import { ContextProvider } from './redux/context';

const theme = createMuiTheme({
    palette: {
        primary: { main: green[700] },
        secondary: { main: red["A700"] },
    },
});

export const BACKEND_ROOT_URL = process.env.NODE_ENV === 'production' ? "https://doshtrade.com" : "http://localhost:8080";
export const WS_BACKEND_ROOT_URL = process.env.NODE_ENV === 'production' ? "wss://doshtrade.com" : "ws://localhost:8080";

const httpLink = new HttpLink({
    uri: `${BACKEND_ROOT_URL}/graphql`,
    // uri: 'http://192.168.1.95:8001/graphql',
});

const wsLink = new WebSocketLink({
    uri: `${WS_BACKEND_ROOT_URL}/graphql`,
    options: {
        reconnect: true
    }
});


const splitLink = split(
    ({ query }) => {
        const definition = getMainDefinition(query);
        return (
            definition.kind === 'OperationDefinition' &&
            definition.operation === 'subscription'
        );
    },
    wsLink,
    httpLink,
);

const client = new ApolloClient({
    cache: new InMemoryCache(),
    link: splitLink,
});


function App() {
    useEffect(() => {
        registerServiceWorker.register();
    }, [])

    return (
        <ApolloProvider client={client}>
            <ThemeProvider theme={theme}>
                <ContextProvider>
                    <CssBaseline />
                    <Router>
                        <StockListContainer />
                    </Router>
                </ContextProvider>
            </ThemeProvider>
        </ApolloProvider>
    );
}

export default App;
