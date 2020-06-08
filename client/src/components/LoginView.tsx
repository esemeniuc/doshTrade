import React, {useState} from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import {Link as RouterLink} from "react-router-dom";
import Container from '@material-ui/core/Container';
import TextField from '@material-ui/core/TextField';
import {Avatar, Box, Button, Link, Paper, Typography} from '@material-ui/core';
import Alert from '@material-ui/lab/Alert';
import {MutationFunctionOptions} from "@apollo/client";
import {LockOutlined} from '@material-ui/icons';
import {LoginMutation} from "../graphql/__generated__/LoginMutation";


const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            '& > *': {
                margin: theme.spacing(1),
                width: '25ch',
                display: 'flex',
                flexDirection: 'column'
            },
        },
        paper: {
            marginTop: theme.spacing(8),
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'center',
        },
        avatar: {
            margin: theme.spacing(1),
            backgroundColor: theme.palette.primary.main,
        },
        form: {
            width: '100%', // Fix IE 11 issue.
            marginTop: theme.spacing(3),
        },
        submit: {
            margin: theme.spacing(3, 0, 2),
        },
    }),
);

interface LoginViewProps {
    onSubmit: (options: MutationFunctionOptions<LoginMutation>) => void;
    loginFailed: boolean;
}

function LoginView(props: LoginViewProps) {
    const classes = useStyles();
    const {onSubmit, loginFailed} = props;
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");

    let handleSubmit = () => {
        onSubmit({
            variables: {
                email: email,
                password: password,
            }
        });
    };
    return (
        <Container component="main" maxWidth="sm">
            <Paper className={classes.paper}>
                <Avatar className={classes.avatar}>
                    <LockOutlined/>
                </Avatar>
                <Typography component="h1" variant="h5" gutterBottom>
                    Sign in to Decloak
                </Typography>
                <form className={classes.root} onSubmit={e => {
                    e.preventDefault();
                    handleSubmit();
                }}>
                    <TextField value={email}
                               onChange={e => setEmail(e.target.value)}
                               id="outlined-basic"
                               label="Email"
                               type="email"
                               variant="outlined"
                    />
                    <TextField value={password}
                               onChange={e => setPassword(e.target.value)}
                               id="outlined-basic"
                               label="Password"
                               type="password"
                               variant="outlined"
                    />

                    <Box p={1}/>

                    <Button variant="contained"
                            color="primary"
                            type="submit"
                            onClick={handleSubmit}>
                        Login
                    </Button>
                </form>

                {loginFailed &&
                <Alert variant="outlined" severity="error">
                    Incorrect email or password
                </Alert>}
                <Link component={RouterLink} to="/signup" variant="button" color="inherit">
                    Sign up
                </Link>
                <Box p={2}/>
            </Paper>
        </Container>
    );
}

export default LoginView;