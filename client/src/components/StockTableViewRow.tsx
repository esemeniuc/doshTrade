import { StockData, Column } from "./StockTableView";
import React from "react";
import { TableCell, Typography, Chip, TableRow, IconButton, makeStyles, Theme, createStyles } from "@material-ui/core";
import { Notifications, NotificationsNone, NotificationsOff } from '@material-ui/icons';
import { AppContext } from "../redux/context";
import { pushPermissionRequest, tickerSubscribe, tickerUnsubscribe } from "../redux/actions";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        mutedButton: {
            color: 'gray'
        },
        activeButton: {
            color: 'red'
        },
    }),
);


function BellButton({ ticker }: { ticker: string }) {
    const { state: { stockSubscriptionState: { tickers }, pushState: { userConsent, subscription } }, dispatch } = React.useContext(AppContext)
    const classes = useStyles();

    // TODO: handle these states somewhere
    // if (error) {
    //     return (<IconButton style={{ color: 'gray' }} onClick={() => { alert("error! " + error) }}><NotificationsOff /></IconButton>)
    // } else if (!pushNotificationSupported) {
    //     return (<IconButton style={{ color: 'gray' }} onClick={() => { alert("push not supported") }}><NotificationsOff /></IconButton>)
    // } 
    if (userConsent === 'default') {
        return (<IconButton className={classes.mutedButton} onClick={() => { dispatch(pushPermissionRequest()) }}><NotificationsNone /></IconButton>)
    } else if (userConsent === 'denied') {
        return (<IconButton className={classes.mutedButton} onClick={() => { alert("push permission is denied") }}><NotificationsOff /></IconButton>)
    } else if (!subscription) {
        return (<IconButton className={classes.activeButton} onClick={() => { dispatch(pushPermissionRequest()) }}><NotificationsNone /></IconButton>)
    } else if (!tickers.includes(ticker)) {
        return (<IconButton className={classes.activeButton} onClick={() => { dispatch(tickerSubscribe(ticker)) }}><NotificationsNone /></IconButton>)
    } else {
        return (<IconButton className={classes.activeButton} onClick={() => { dispatch(tickerUnsubscribe(ticker)) }}><Notifications /></IconButton>)
    }
}

function RsiCellContent(column: Column, value: string | number) {
    return (
        <TableCell key={column.id} align={column.align}>
            <Typography variant="subtitle2">
                {value}
            </Typography>
        </TableCell>
    )
}

function TickerCellContent(column: Column, value: string | number) {
    return (
        <TableCell key={column.id} align={column.align}>
            <Typography variant="h6">
                {value}
            </Typography>
        </TableCell>
    )
}

function PriceCellContent(column: Column, value: string | number) {
    return (
        <TableCell key={column.id} align={column.align}>
            <Typography variant="subtitle2">
                {value}
            </Typography>
        </TableCell>)
}

function SinceOpenCellContent(column: Column, value: number) {
    const plusSign = value > 0 ? '+' : ''
    return (
        <TableCell key={column.id} align={column.align}>
            <Chip label={`${plusSign + value.toFixed(2) + '%'}`} color='primary' />
        </TableCell>)
}

function StockTableViewRow(row: StockData, columns: Column[]) {
    return (
        <TableRow hover role={"checkbox"} tabIndex={-1} key={row.code}>
            <TableCell><BellButton ticker={row.ticker} /></TableCell>
            {columns.map((column) => {
                return cellContent(row, column)
            })}
        </TableRow>)
}

function cellContent(rowData: StockData, column: Column) {
    const value = rowData[column.id];
    switch (column.id) {
        case 'rsi':
            return RsiCellContent(column, value)
        case 'ticker':
            return TickerCellContent(column, value)
        case 'price':
            return PriceCellContent(column, value)
        case 'sinceOpen':
            return SinceOpenCellContent(column, value as number)
        default:
            return (
                <TableCell key={column.id} align={column.align}>
                    {column.format && typeof value === 'number' ? column.format(value) : value}
                </TableCell>
            );
    }
}

export default StockTableViewRow;