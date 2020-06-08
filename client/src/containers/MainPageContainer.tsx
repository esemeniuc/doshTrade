import React, {useState} from 'react';
import DashboardView from '../components/DashboardView';
import {Redirect, Route, Switch} from "react-router-dom";
import {useQuery} from '@apollo/client';
import LoginContainer from './LoginContainer';
import SignUpContainer from './SignUpContainer';
import HomeContainer from "./HomeContainer";
import {loader} from 'graphql.macro';
import {AuthContext} from '../useAuth';
import {IsAuth} from "../graphql/__generated__/IsAuth";

const ISAUTH_QUERY = loader('../graphql/isAuth.gql');

function MainPageContainer() {
    const [authToken, setAuthToken] = useState(localStorage.getItem("authToken"));
    const {data, loading} = useQuery<IsAuth>(ISAUTH_QUERY,
        {
            variables: {jwt: localStorage.getItem("authToken")},
            onCompleted: (result) => {
                if (!result.isAuth) {
                    localStorage.removeItem("authToken");
                    setAuthToken(null);
                }
            },
            // fetchPolicy: "cache-and-network",
        });

    if (loading) return <div>loading...</div>;
    if (!data) return <>Error!</>;

    if (authToken) { // authenticated
        //need this auth provider since top bar consumes it
        return <AuthContext.Provider value={{authToken, setAuthToken}}>
            <Switch>
                <Route path={["/", "/login", "/signup"]} exact>
                    <Redirect to="/dashboard"/>
                </Route>

                <Route path="/dashboard" exact>
                    <DashboardView/>
                </Route>
            </Switch>
        </AuthContext.Provider>;
    } else { // not authenticated
        //need this auth provider for login to set token
        return <AuthContext.Provider value={{authToken, setAuthToken}}>
            <Switch>
                <Route path="/" exact>
                    <HomeContainer/>
                </Route>

                <Route path="/login" exact>
                    <LoginContainer/>
                </Route>

                <Route path="/signup" exact>
                    <SignUpContainer/>
                </Route>

                <Route>
                    <Redirect to="/"/>
                </Route>
            </Switch>
        </AuthContext.Provider>;
    }
}

export default MainPageContainer;