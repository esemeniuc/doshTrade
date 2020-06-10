import React from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import Container from "@material-ui/core/Container";
import {
    AppBar,
    Avatar,
    Box, Chip,
    List,
    ListItem,
    ListItemAvatar,
    ListItemSecondaryAction,
    ListItemText,
    Paper, Toolbar, Typography
} from "@material-ui/core";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
        },
        menuButton: {
            marginRight: theme.spacing(2),
        },
        list: {
            marginTop: theme.spacing(4),
            width: '100%',
            backgroundColor: theme.palette.background.paper
        }
    }),
);

function StockListView() {
    const classes = useStyles();

    return (
        <div className={classes.root} >
            <Container component="main" maxWidth='sm'>
                <AppBar position="static">
                    <Toolbar>
                        <Typography variant="h6" >
                            Yolo Trader
                        </Typography>
                    </Toolbar>
                </AppBar>
                    <Typography variant="caption" >
                        <Box textAlign="center" >
                            updated since june 10, 2020 12:00:15PM
                        </Box>
                    </Typography>
                    <List className={classes.list}>
                        <ListItem button>
                            <ListItemText id='{1}' primary='TSLA' secondary='Up 7.86%'/>
                            <ListItemSecondaryAction>
                                <Chip label="1011.87" color='primary'/>
                            </ListItemSecondaryAction>
                        </ListItem>

                        <ListItem button>
                            <ListItemText id='{2}' primary='SQ' secondary='Up 2.51'/>
                            <ListItemSecondaryAction>
                                <Chip label="92.12" color='primary'/>
                            </ListItemSecondaryAction>
                        </ListItem>

                        <ListItem button>
                            <ListItemText id='{3}' primary='SPY' secondary='Up 0.12'/>
                            <ListItemSecondaryAction>
                                <Chip label="320.87" color='primary'/>
                            </ListItemSecondaryAction>
                        </ListItem>


                    </List>
            </Container>
        </div>
    );
}

export default StockListView;