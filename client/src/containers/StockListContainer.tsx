import React from 'react';
import Container from "@material-ui/core/Container";
import {
    AppBar,
    Box,
    Toolbar, Typography
} from "@material-ui/core";

function StockListContainer() {

    return (
        <Container component="main" maxWidth='sm'>
            <AppBar position="static">
                <Toolbar>
                    <Typography variant="h6">
                        Yolo Trader
                    </Typography>
                </Toolbar>
            </AppBar>
            <Typography variant="caption">
                <Box textAlign="center">
                    Since close yesterday
                </Box>
            </Typography>
        </Container>
    );
}


export default StockListContainer;