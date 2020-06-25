import { StockData, Column } from "./StockTableView";
import React from "react";
import { TableCell, Typography, Chip, TableRow, Button } from "@material-ui/core";
import usePushNotifications, {PushContext} from "../push/usePushNotifications";
import { Notifications, NotificationsNone, NotificationsOff} from '@material-ui/icons';
import useLocalStorage from '../push/useLocalStorage'

function BellButton({ ticker }: { ticker: String }) {
    const {
        pushNotificationSupported,
        userSubscription,
        onClickAskUserPermission,
        onClickSubscribeToPushNotification,
        pushServerSubscriptionId,
        error,
        loading
        } = usePushNotifications();
    const [subscribedTickers, setSubscribedTickers] = useLocalStorage('subscribedTickers', []);
    const pushContext = React.useContext(PushContext)
    
    let BellIcon;
    let color;
    let handler;
    if (error) {
        BellIcon = NotificationsOff
        color = 'gray'
        handler = () => { alert("error! " + error)}
    } else if (!pushNotificationSupported) {
        BellIcon = NotificationsOff
        color = 'gray'
        handler = () => { alert("push not supported")}
    } else if (pushContext.userConsent === 'default') {
        BellIcon = NotificationsNone
        color = 'gray'
        handler = () => {
            onClickAskUserPermission().then(() => {
                onClickSubscribeToPushNotification()
            })
        }
    } else if (pushContext.userConsent === 'denied') {
        BellIcon = NotificationsOff
        color = 'gray'
        handler = () => {alert("push permission denied, should turn it on to enable push")}
    } else if (!userSubscription) {
        BellIcon = NotificationsNone
        color = 'gray'
        handler = () => {
            onClickSubscribeToPushNotification()
        }
    } else if (!subscribedTickers.includes(ticker)) {
        BellIcon = NotificationsNone
        color = 'red'
        handler = () => {
            // TODO: send tickers to server
            setSubscribedTickers([...subscribedTickers, ticker])
        }
    } else {
        BellIcon = Notifications
        color = 'red'
        handler = () => {
            // TODO: send tickers to server
            console.log(subscribedTickers)
            const removed = subscribedTickers.filter((t:String) => t !== ticker)
            setSubscribedTickers(removed)
        }
    }

    return (<Button style={{color:color}} onClick={handler}><BellIcon></BellIcon></Button>)
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
            <Chip label={`${value}`} color='primary'/>
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
    return(
        <TableRow hover role={"checkbox"} tabIndex={-1} key={row.code}>
            <TableCell><BellButton ticker={row.ticker}/></TableCell>
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