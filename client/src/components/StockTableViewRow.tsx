import { StockData, Column } from "./StockTableView";
import React from "react";
import { TableCell, Typography, Chip, TableRow, Button } from "@material-ui/core";
import { Notifications, NotificationsNone, NotificationsOff } from '@material-ui/icons';
import { AppContext } from "../redux/context";
import { pushPermissionRequest, tickerSubscribe, tickerUnsubscribe } from "../redux/actions";

function BellButton({ ticker }: { ticker: string }) {
    const { state: { stockSubscriptionState: { tickers }, pushState: { userConsent, subscription } }, dispatch } = React.useContext(AppContext)
    // TODO: handle these states somewhere
    // if (error) {
    //     return (<Button style={{ color: 'gray' }} onClick={() => { alert("error! " + error) }}><NotificationsOff /></Button>)
    // } else if (!pushNotificationSupported) {
    //     return (<Button style={{ color: 'gray' }} onClick={() => { alert("push not supported") }}><NotificationsOff /></Button>)
    // } 
    if (userConsent === 'default') {
        return (<Button style={{ color: 'gray' }} onClick={() => { dispatch(pushPermissionRequest()) }}><NotificationsNone /></Button>)
    } else if (userConsent === 'denied') {
        return (<Button style={{ color: 'gray' }} onClick={() => { alert("push permission is denied") }}><NotificationsOff /></Button>)
    } else if (!subscription) {
        return (<Button style={{ color: 'red' }} onClick={() => { dispatch(pushPermissionRequest()) }}><NotificationsNone /></Button>)
    } else if (!tickers.includes(ticker)) {
        return (<Button style={{ color: 'red' }} onClick={() => { dispatch(tickerSubscribe(ticker)) }}><NotificationsNone /></Button>)
    } else {
        return (<Button style={{ color: 'red' }} onClick={() => { dispatch(tickerUnsubscribe(ticker)) }}><Notifications /></Button>)
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
            <Chip label={`${value}`} color='primary' />
        </TableCell>)
}

function SinceOpenCellContent(column: Column, value: number) {
    const plusSign = value > 0 ? '+' : ''
    return (
        <TableCell key={column.id} align={column.align}>
            <Typography variant="subtitle1">
                {plusSign + value.toFixed(2) + '%'}
            </Typography>
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