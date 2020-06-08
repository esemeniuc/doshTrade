import React, {useState} from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import Container from '@material-ui/core/Container';
import TextField from '@material-ui/core/TextField';
import {Avatar, Box, Button, Link, Paper, Typography} from '@material-ui/core';
import {SignupMutation} from '../graphql/__generated__/SignupMutation';
import {MutationFunctionOptions} from '@apollo/client';
import Alert from '@material-ui/lab/Alert';
import {PostAdd} from "@material-ui/icons";
import {Link as RouterLink} from "react-router-dom";

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

export interface SignUpViewProps {
    onSubmit: (options: MutationFunctionOptions<SignupMutation>) => void;
    signUpFailed: boolean;
}

function SignUpView(props: SignUpViewProps) {
    const classes = useStyles();
    const {onSubmit, signUpFailed} = props;
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [lastName, setLastName] = useState("");
    const [firstName, setFirstName] = useState("");

    const handleSubmit = () => {
        onSubmit({
            variables: {
                firstName: firstName,
                lastName: lastName,
                email: email,
                password: password,
            }
        });
    };

    return (
        <Container component="main" maxWidth="sm">
            <Paper className={classes.paper}>
                <Avatar className={classes.avatar}>
                    <PostAdd/>
                </Avatar>
                <Typography component="h1" variant="h5" gutterBottom>
                    Sign up for Decloak
                </Typography>
                <form className={classes.root} onSubmit={e => {
                    e.preventDefault();
                    handleSubmit();
                }}>
                    <TextField required
                               value={firstName}
                               onChange={e => setFirstName(e.target.value)}
                               id="outlined-basic"
                               label="First name"
                               type="text"
                               variant="outlined"
                    />
                    <TextField required
                               value={lastName}
                               onChange={e => setLastName(e.target.value)}
                               id="outlined-basic"
                               label="Last name"
                               type="text"
                               variant="outlined"
                    />
                    <TextField required
                               value={email}
                               onChange={e => setEmail(e.target.value)}
                               id="outlined-basic"
                               label="Email"
                               type="email"
                               variant="outlined"
                    />
                    <TextField required
                               value={password}
                               onChange={e => setPassword(e.target.value)}
                               id="outlined-basic"
                               label="Password"
                               type="password"
                               helperText="8+ characters"
                               variant="outlined"
                    />

                    <Box p={1}/>

                    <Button variant="contained"
                            color="primary"
                            type="submit"
                            onClick={handleSubmit}>
                        Sign Up
                    </Button>
                </form>

                {signUpFailed &&
                <Alert variant="outlined" severity="error">
                    Looks like you already signed up
                </Alert>}

                <Link component={RouterLink} to="/login" variant="button" color="inherit">
                    Login
                </Link>
                <Box p={2}/>
            </Paper>
        </Container>
    );
}

export default SignUpView;