import React, {useContext, useState} from 'react';
import {AppBar, Button, Dialog, DialogActions, DialogContent, IconButton, Toolbar, Typography} from '@material-ui/core';
import {createStyles, makeStyles} from '@material-ui/core/styles';
import {useHistory} from 'react-router'
// import {DRAWER_WIDTH} from '../constants';
import {Close, Help} from "@material-ui/icons";
import MuiDialogTitle from '@material-ui/core/DialogTitle';
import {useApolloClient} from "@apollo/client";
import {AuthContext} from "../useAuth";

const useStyles = makeStyles(theme =>
    createStyles({
        appBar: {
            // width: `calc(100% - ${DRAWER_WIDTH}px)`,
            // marginLeft: DRAWER_WIDTH,
        },
        title: {
            flexGrow: 1,
        },
        root: {
            margin: 0,
            padding: theme.spacing(2),
        },
        closeButton: {
            position: 'absolute',
            right: theme.spacing(1),
            top: theme.spacing(1),
            color: theme.palette.grey[500],
        },
    }),
);

export default function TopBar() {
    const classes = useStyles();
    const history = useHistory();
    const client = useApolloClient();
    const {setAuthToken} = useContext(AuthContext);
    return <AppBar position="static" className={classes.appBar}>
        <Toolbar>
        <Typography variant="h6" className={classes.title}>
        Decloak
        </Typography>
        <Button color="inherit"
        onClick={() => {
            localStorage.removeItem("authToken");
            setAuthToken && setAuthToken(null);
            history.push('/');
            client.clearStore();
        }}>
        Logout
        </Button>
        </Toolbar>
    </AppBar>;
}
