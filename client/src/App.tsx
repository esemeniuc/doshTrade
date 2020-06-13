import React from 'react';
import {BrowserRouter as Router} from "react-router-dom";
import {ApolloClient, ApolloProvider, HttpLink, InMemoryCache} from '@apollo/client';
import {setContext} from '@apollo/link-context';
import {createMuiTheme, CssBaseline, ThemeProvider} from "@material-ui/core";
import {green, red} from "@material-ui/core/colors";
import MainPageContainer from './containers/MainPageContainer';
import StockTableView from "./components/StockTableView";

const authLink = setContext((_, {headers}) => {
    // get the authentication token from local storage if it exists
    const token = localStorage.getItem('authToken');
    // return the headers to the context so httpLink can read them
    return {
        headers: {
            ...headers,
            authorization: token ? `Bearer ${token}` : "",
        }
    };
});

const theme = createMuiTheme({
    palette: {
        primary: {main: green[700]},
        secondary: {main: red["A700"]},
    },
});

const client = new ApolloClient({
    cache: new InMemoryCache(),
    link: authLink.concat(new HttpLink({
        uri: 'http://localhost:8000/graphql',
        // uri: 'http://192.168.1.95:8001/graphql',
    })),
});

function App() {
    return (
        <ApolloProvider client={client}>
            <ThemeProvider theme={theme}>
                <CssBaseline/>
                <Router>
                    <StockTableView/>
                </Router>
            </ThemeProvider>
        </ApolloProvider>
    );
}

export default App;
