import React from 'react';
import {BrowserRouter as Router} from "react-router-dom";
import MainPageContainer from './containers/MainPageContainer';
import {ApolloClient, ApolloProvider, HttpLink, InMemoryCache} from '@apollo/client';
import {setContext} from '@apollo/link-context';
import {createMuiTheme, CssBaseline, ThemeProvider} from "@material-ui/core";
import {green, red} from "@material-ui/core/colors";

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

// class DebugRouter extends Router {
//     history:any;
//     constructor(props: BrowserRouterProps) {
//         super(props);
//         console.log('initial history is: ', JSON.stringify(this.history, null, 2));
//         //@ts-ignore
//         this.history.listen((location, action) => {
//             console.log(
//                 `The current URL is ${location.pathname}${location.search}${location.hash}`
//             );
//             console.log(`The last navigation action was ${action}`, JSON.stringify(this.history, null, 2));
//         });
//     }
// }

function App() {
    return (
        <ApolloProvider client={client}>
            <ThemeProvider theme={theme}>
                <CssBaseline/>
                <Router>
                    <MainPageContainer/>
                </Router>
            </ThemeProvider>
        </ApolloProvider>
    );
}

export default App;
