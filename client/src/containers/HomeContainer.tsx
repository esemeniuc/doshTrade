import React from "react";
import {AppBar, Box, Button, Grid, Theme, Toolbar, Typography} from "@material-ui/core";
import {AddToQueue, AssignmentTurnedIn, CheckCircleOutline, Home, SvgIconComponent} from '@material-ui/icons';
import Link from '@material-ui/core/Link';
import {ReactComponent as FaxtailBanner} from './decloak.svg';
import {Link as RouterLink, useHistory} from 'react-router-dom';
import {createStyles, makeStyles} from "@material-ui/core/styles";

function TopBar() {
    const useStyles = makeStyles((theme: Theme) =>
        createStyles({
            title: {
                flexGrow: 1,
            },
        }),
    );
    const classes = useStyles();
    return <AppBar position="fixed" >
        <Toolbar>
            <Typography variant="h6" className={classes.title}>
                Decloak
            </Typography>
            <Link component={RouterLink} to="/login" variant="button" color="inherit">
                Login
            </Link>
            <Box p={1}/>
            <Link component={RouterLink} to="/signup" variant="button" color="inherit">
                Sign up
            </Link>
        </Toolbar>
    </AppBar>;
}

export default function HomeContainer() {
    const history = useHistory();
    return <>
        <TopBar/>
        <Box p={3}/>
        <Box p={4} mt={3}>
            <Grid container spacing={6} justify="center" alignItems="center">
                <Grid item>
                    <Typography variant="h4" component="h1" color="textPrimary" gutterBottom>
                        Simple Incognito Detection
                    </Typography>
                    <Typography variant="subtitle1" component="h3" color="textSecondary">
                        Detect when your users are incognito
                    </Typography>
                </Grid>
                <Grid item>
                    <FaxtailBanner width={300} height={200}/>
                </Grid>
            </Grid>

            <Box p={4}/>

            <Box display="flex" flexDirection="column" alignItems="center">
                <Button size="large"
                        style={{fontSize: "1.2rem"}}
                        variant="contained"
                        color="primary"
                        onClick={() => history.push("/signup")}>
                    Get Started
                </Button>
            </Box>

            <Box p={2}/>

            <blockquote>
                <Box my={12}>
                    <Typography variant="h5" color="textPrimary" align='center'
                                style={{justifySelf: "center"}}>
                        Incognito recognition service that shows how many users are visiting your website in incognitio mode
                    </Typography>
                </Box>
            </blockquote>

            <Box my={10}>
                <Grid container spacing={6}>
                    <Grid item xs style={{alignSelf: "center"}}>
                        <Typography variant="h5" component="h1" color="textPrimary" align="center" gutterBottom>
                            Using Decloak is Easy
                        </Typography>
                    </Grid>
                    {
                        [
                            {
                                title: "1. Sign up",
                                icon: AssignmentTurnedIn
                            },
                            {
                                title: "2. Add your website",
                                icon: Home
                            },
                            {
                                title: "3. Install Decloak package",
                                icon: AddToQueue
                            },
                            {
                                title: "4. See results",
                                icon: CheckCircleOutline
                            },
                        ].map((props: { title: string, icon: SvgIconComponent }, idx) =>
                            <Grid item xs={12} sm key={idx}>
                                <Box display="flex" flexDirection="column" alignItems="center">
                                    <props.icon style={{fontSize: 64, textAlign: "center"}} color="secondary"/>
                                    <Typography variant="body1" align="center" color="textSecondary">
                                        {props.title}
                                    </Typography>
                                </Box>
                            </Grid>)
                    }
                </Grid>
            </Box>
        </Box>
    </>;
}
