import React, {useContext, useState} from 'react';
import {useHistory} from "react-router-dom";
import LoginView from '../components/LoginView';
import {ApolloError, useMutation} from '@apollo/client';
import {loader} from 'graphql.macro';
import {LoginMutation} from '../graphql/__generated__/LoginMutation';
import {AuthContext} from "../useAuth";

const LOGIN_MUTATION = loader('../graphql/login.gql');

function isAuthError(error: ApolloError) {
    const { graphQLErrors } = error;
    return graphQLErrors && ["AUTHENTICATION_ERROR", "AUTHORIZATION_ERROR"].includes(graphQLErrors[0]?.message);
}
const ISAUTH_QUERY = loader('../graphql/isAuth.gql');

function LoginContainer() {
    const {setAuthToken} = useContext(AuthContext);
    const [loginFailed, setLoginFailed] = useState<boolean>(false)
    const history = useHistory();
    const [submitLogin, { loading }] = useMutation<LoginMutation>(LOGIN_MUTATION, {
        onCompleted: (data) => {
            if (data.login && setAuthToken) {
                localStorage.setItem('authToken', data.login);
                setAuthToken(data.login);
                history.push('/dashboard'); //on login, go to dashboard
            }
        },
        onError: (error) => {
            isAuthError(error) && setLoginFailed(true)
        },
        refetchQueries: [{query: ISAUTH_QUERY}],
    });

    if (loading) return <div>loading...</div>;

    return <LoginView onSubmit={submitLogin} loginFailed={loginFailed} />;
}

export default LoginContainer;