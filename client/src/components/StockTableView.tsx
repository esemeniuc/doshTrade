import React from 'react';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import {
    Paper
} from "@material-ui/core";
import StockTableViewRow from './StockTableViewRow'

import TableSortLabel from '@material-ui/core/TableSortLabel';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import { Order, stableSort, getComparator } from '../util/sort'
import { Column, StockData } from './StockTableTypes';

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
        },
        visuallyHidden: {
            border: 0,
            clip: 'rect(0 0 0 0)',
            height: 1,
            margin: -1,
            overflow: 'hidden',
            padding: 0,
            position: 'absolute',
            top: 20,
            width: 1,
        },
    }),
);

export const columns: Column[] = [
    { id: 'ticker', numeric: false, label: 'Ticker', minWidth: 50 },
    {
        id: 'price',
        numeric: true,
        label: 'Price',
        minWidth: 100,
        format: (value: number) => value.toLocaleString('en-US'),
    },
    {
        id: 'sinceOpen',
        numeric: true,
        label: 'Since open',
        minWidth: 100,
    },
    {
        id: 'rsi',
        numeric: true,
        label: 'RSI',
        minWidth: 80,
        format: (value: number) => value.toFixed(2),
    },
];

interface EnhancedTableProps {
    classes: ReturnType<typeof useStyles>;
    onRequestSort: (event: React.MouseEvent<unknown>, property: keyof StockData) => void;
    order: Order;
    orderBy: keyof StockData | undefined;
}

function EnhancedStockTableHead(props: EnhancedTableProps) {
    const { classes, order, orderBy, onRequestSort } = props;
    const createSortHandler = (property: keyof StockData) => (event: React.MouseEvent<unknown>) => {
        onRequestSort(event, property);
    };

    return (
        <TableHead>
            <TableRow>
                <TableCell
                    key={'notification'}
                    align={'left'}
                    padding={'none'}
                    style={{ minWidth: 30 }}
                >
                </TableCell>
                {columns.map((col) => (
                    <TableCell
                        key={col.id}
                        align={col.numeric ? 'right' : 'left'}
                        sortDirection={orderBy === col.id ? order : false}
                    >
                        <TableSortLabel
                            active={orderBy === col.id}
                            direction={orderBy === col.id ? order : 'asc'}
                            onClick={createSortHandler(col.id)}
                        >
                            {col.label}
                            {orderBy === col.id ? (
                                <span className={classes.visuallyHidden}>
                                    {order === 'desc' ? 'sorted descending' : 'sorted ascending'}
                                </span>
                            ) : null}
                        </TableSortLabel>
                    </TableCell>
                ))}
            </TableRow>
        </TableHead>
    );
}

function StockTableView({ stockData }: { stockData: StockData[] }) {
    const classes = useStyles()
    const [order, setOrder] = React.useState<Order>('asc');
    const [orderBy, setOrderBy] = React.useState<keyof StockData | undefined>(undefined);

    const handleRequestSort = (_: React.MouseEvent<unknown>, property: keyof StockData) => {
        // cycle through 'asc', 'desc', and undefined
        if (orderBy === undefined) {
            setOrder('asc');
            setOrderBy(property);
        } else {
            if (order === 'asc') {
                setOrder('desc');
                setOrderBy(property);
            } else {
                setOrder('asc');
                setOrderBy(undefined);
            }
        }

    };

    return (
        <TableContainer component={Paper}>
            <Table stickyHeader aria-label="sticky table">
                <EnhancedStockTableHead
                    classes={classes}
                    order={order}
                    orderBy={orderBy}
                    onRequestSort={handleRequestSort}
                />
                <TableBody>
                    {orderBy ?
                        stableSort(stockData, getComparator(order, orderBy))
                            .map(row => <StockTableViewRow row={row} columns={columns} key={row.ticker} />) :
                        stockData
                            .map(row => <StockTableViewRow row={row} columns={columns} key={row.ticker} />)}
                </TableBody>
            </Table>
        </TableContainer>

    );
}

export default StockTableView;