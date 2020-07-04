import React from 'react';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Container from "@material-ui/core/Container";
import {
    AppBar, Box,
    Chip,
    Paper, Toolbar, Typography
} from "@material-ui/core";
import StockTableViewRow from './StockTableViewRow'

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

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

export interface Column {
    id: 'ticker' | 'code' | 'price' | 'sinceOpen' | 'rsi';
    label: string;
    minWidth?: number;
    format?: (value: number) => string;
}

export const columns: Column[] = [
    { id: 'ticker', label: 'Ticker', minWidth: 50 },
    {
        id: 'price',
        label: 'Price',
        minWidth: 100,
        format: (value: number) => value.toLocaleString('en-US'),
    },
    {
        id: 'sinceOpen',
        label: 'Since open',
        minWidth: 100,
    },
    {
        id: 'rsi',
        label: 'RSI',
        minWidth: 80,
        format: (value: number) => value.toFixed(2),
    },
];

export interface StockData {
    ticker: string;
    code: string;
    price: number;
    sinceOpen: number;
    rsi: string;
}

function StockTableHead() {
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
                {columns.map((column) => (
                    <TableCell
                        key={column.id}
                        align={'left'}
                        style={{ minWidth: column.minWidth }}
                    >
                        {column.label}
                    </TableCell>
                ))}
            </TableRow>
        </TableHead>
    )
}

function StockTableView({ stockData }: { stockData: StockData[] }) {
    const classes = useStyles()
    return (
        <TableContainer component={Paper}>
            <Table stickyHeader aria-label="sticky table">
                <StockTableHead />
                <TableBody>
                    {stockData.map(row => <StockTableViewRow row={row} columns={columns} />)}
                </TableBody>
            </Table>
        </TableContainer>

    );
}

export default StockTableView;