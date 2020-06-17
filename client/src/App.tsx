import React from 'react';
import {BrowserRouter as Router} from "react-router-dom";
import {ApolloClient, ApolloProvider, HttpLink, InMemoryCache, split} from '@apollo/client';
import {createMuiTheme, CssBaseline, ThemeProvider} from "@material-ui/core";
import {green, red} from "@material-ui/core/colors";
import { getMainDefinition } from '@apollo/client/utilities';
import { WebSocketLink } from '@apollo/link-ws';
import StockListContainer from "./containers/StockListContainer";

const theme = createMuiTheme({
    palette: {
        primary: {main: green[700]},
        secondary: {main: red["A700"]},
    },
});

const httpLink = new HttpLink({
    uri: 'http://localhost:8080/graphql',
    // uri: 'http://192.168.1.95:8001/graphql',
});

const wsLink = new WebSocketLink({
    uri: `ws://localhost:8080/graphql`,
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
    return (
        <ApolloProvider client={client}>
            <ThemeProvider theme={theme}>
                <CssBaseline/>
                <Router>
                    <StockListContainer />
                </Router>
            </ThemeProvider>
        </ApolloProvider>
    );
}

export default App;
