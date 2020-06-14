import React from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';

import TopBar from './TopBar';
import PrivateModeStatsView from './PrivateModeStatsView';
import {useQuery} from "@apollo/client";
import {GetPropertiesQuery} from "./__generated__/GetPropertiesQuery";
import {loader} from "graphql.macro";
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

const GETPROPERTIES_QUERY = loader('../graphql/getProperties.gql');

function DashboardView() {
    const classes = useStyles();
    const {loading, error, data} = useQuery<GetPropertiesQuery>(GETPROPERTIES_QUERY);
    if (loading) return <>Loading Dashboard!</>;
    if (error) return <>{`Error! ${error}`}</>;
    if (!data) return <>Error! no data</>;

    return (
        <div className={classes.root}>
            <TopBar/>
            <Container component="main">
                <Paper>
                    <Box p={3}>
                        {
                            data.getProperties.length > 0 ?
                                <PrivateModeStatsView property={data.getProperties[0]}/> : <div/>
                        }
                    </Box>
                </Paper>
            </Container>
        </div>
    );
}

export default DashboardView;