import React, {useState} from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import {Bar, BarChart, CartesianGrid, Label, Legend, Tooltip, XAxis, YAxis} from 'recharts';
import {useQuery} from "@apollo/client";
import {loader} from "graphql.macro";
import {Box, Link, TextField, Typography} from "@material-ui/core";
import dayjs from "dayjs";
import {PrivateModeStatsByDateQuery} from "../graphql/__generated__/PrivateModeStatsByDateQuery";
import {GetPropertiesQuery_getProperties} from "./__generated__/GetPropertiesQuery";

const PRIVATEMODESTATSBYDATE_QUERY = loader('../graphql/privateModeStatsByDate.gql');

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        textField: {
            marginLeft: theme.spacing(1),
            marginRight: theme.spacing(1),
        },
    }),
);

function PrivateModeStatsView(props: { property: GetPropertiesQuery_getProperties }) {
    const classes = useStyles();
    const [startDate, setStartDate] = useState(dayjs().subtract(2, 'week').format('YYYY-MM-DD'));
    const [endDate, setEndDate] = useState(dayjs().format("YYYY-MM-DD"));
    const {data, loading, error} = useQuery<PrivateModeStatsByDateQuery>(PRIVATEMODESTATSBYDATE_QUERY,
        {variables: {propertyId: props.property.id, startDate, endDate}});
    if (loading) return <>Loading Stats!</>;
    if (error) return <>{`Error! ${error}`}</>;
    if (!data) return <>Error! no data</>;

    const filteredData = (data.privateModeStatsByDate ?? []).filter(e => e.privateCount > 0 || e.nonPrivateCount > 0);
    return <>
        <Typography variant="h4" component="h1">
            {props.property.websiteName}
        </Typography>
        <Link href={props.property.websiteUrl}>
            {props.property.websiteUrl}
        </Link>

        <Box py={1} display="flex" flexDirection="column" alignItems="center" flexWrap="wrap">
            <Box>
                <TextField required
                           label="Start date"
                           type="date"
                           defaultValue={startDate}
                           className={classes.textField}
                           InputLabelProps={{
                               shrink: true,
                           }}
                           onChange={e => setStartDate(e.target.value)}
                />
                <TextField required
                           label="End date"
                           type="date"
                           defaultValue={endDate}
                           className={classes.textField}
                           InputLabelProps={{
                               shrink: true,
                           }}
                           onChange={e => setEndDate(e.target.value)}
                />
            </Box>

            <BarChart
                width={600}
                height={250}
                data={data.privateModeStatsByDate || []}
                margin={{top: 15, right: 15, left: -15, bottom: 15}}
            >
                <CartesianGrid strokeDasharray="3 3">
                    <Label value="Pages of my website" offset={0} position="center"/>
                </CartesianGrid>
                {
                    filteredData.length > 0 ? <XAxis name="Date" dataKey="date"/> :
                        <XAxis>
                            <Label value={'No data yet, please see the onboarding steps 🙂'} position='top'
                                   offset={100}/>
                        </XAxis>
                }
                <YAxis label={{value: "Visits", angle: -90,}}/>

                <Tooltip/>
                <Legend/>
                <Bar dataKey="privateCount" name="Private Visitors" fill="#8884d8"/>
                <Bar dataKey="nonPrivateCount" name="Normal Visitors" fill="#82ca9d"/>
            </BarChart>
        </Box>
    </>;
}

export default PrivateModeStatsView;