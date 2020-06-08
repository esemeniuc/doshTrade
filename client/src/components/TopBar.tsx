import React, {useState} from 'react';
import {Button, Dialog, DialogActions, DialogContent, IconButton, Typography} from '@material-ui/core';
import {createStyles, makeStyles} from '@material-ui/core/styles';
// import {DRAWER_WIDTH} from '../constants';
import {Close, Help} from "@material-ui/icons";
import MuiDialogTitle from '@material-ui/core/DialogTitle';

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

function CustomizedDialogs() {
    const [open, setOpen] = useState(false);
    const handleClose = () => setOpen(false);
    const classes = useStyles();

    return <div>
        <IconButton color="inherit" onClick={() => setOpen(true)}>
            <Help/>
        </IconButton>
        <Dialog onClose={handleClose} open={open}>
            <MuiDialogTitle disableTypography className={classes.root}>
                <Typography variant="h6">
                    Onboarding
                </Typography>
                <IconButton className={classes.closeButton} onClick={handleClose}>
                    <Close/>
                </IconButton>
            </MuiDialogTitle>
            <DialogContent dividers>
                <Typography gutterBottom>
                    This is the tracking code for this property. Copy and paste this code as
                    the first item into the <head> of every webpage you want to track. If you already have a Global Site
                    Tag on your page, simply add the config line from the snippet
                    below to your existing Global Site Tag.
                </Typography>
                <Typography gutterBottom>
                    The Global Site Tag provides streamlined tagging across Google’s site measurement, conversion
                    tracking, and remarketing products – giving you better control while making implementation easier.
                    By using gtag.js, you will be able to benefit from the latest dynamic features and integrations as
                    they become available.
                </Typography>
            </DialogContent>
            <DialogActions>
                <Button autoFocus
                        onClick={handleClose}
                        color="primary">
                    Close
                </Button>
            </DialogActions>
        </Dialog>
    </div>;
;
;
;
;
}

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
    <CustomizedDialogs/>
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
