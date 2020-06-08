import React, {useContext, useState} from 'react';
import {useHistory} from "react-router-dom";
import SignUpView from '../components/SignUpView';
import {ApolloError, useMutation} from '@apollo/client';
import {loader} from 'graphql.macro';
import {SignupMutation} from '../graphql/__generated__/SignupMutation';
import {AuthContext} from "../useAuth";

const SIGN_UP_MUTATION = loader('../graphql/signUp.gql');

function isSignupError(error: ApolloError) {
    const {graphQLErrors} = error;
    return graphQLErrors && ["DB_INSERT_ERROR"].includes(graphQLErrors[0]?.message);
}

const ISAUTH_QUERY = loader('../graphql/isAuth.gql');

function SignUpContainer() {
    const {setAuthToken} = useContext(AuthContext);
    const history = useHistory();
    const [signUpFailed, setSignUpFailed] = useState<boolean>(false);
    const [submitSignUp, {loading}] = useMutation<SignupMutation>(SIGN_UP_MUTATION, {
        onCompleted: (data) => {
            if (data.signup && setAuthToken) {
                localStorage.setItem('authToken', data.signup);
                setAuthToken(data.signup);
                history.push('/dashboard'); //on signup, go to dashboard
            }
        },
        onError: (error) => {
            isSignupError(error) && setSignUpFailed(true);
        },
        refetchQueries: [{
            query: ISAUTH_QUERY,
            variables: {jwt: localStorage.getItem("authToken")},
        }],
        // awaitRefetchQueries: true,
    });

    if (loading) return <div>loading...</div>;

    return <SignUpView onSubmit={submitSignUp} signUpFailed={signUpFailed}/>;
}

export default SignUpContainer;