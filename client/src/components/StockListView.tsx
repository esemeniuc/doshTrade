import React from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import Container from "@material-ui/core/Container";
import {Box, Paper} from "@material-ui/core";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
        },
        menuButton: {
            marginRight: theme.spacing(2),
        }
    }),
);

function StockListView() {
    const classes = useStyles();

    return (
        <div className={classes.root}>
            <Container component="main">
                <Paper>
                    <Box p={3}>
                        stock list view
                    </Box>
                </Paper>
            </Container>
        </div>
    );
}

export default StockListView;