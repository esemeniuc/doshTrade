import React from 'react';
import StockTableViewRow from './StockTableViewRow'

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

export interface Column {
    id: 'ticker' | 'code' | 'price' | 'sinceOpen' | 'rsi';
    label: string;
    minWidth?: number;
    align?: 'right' | 'left' | 'center';
    format?: (value: number) => string;
}

export const columns: Column[] = [
    { id: 'ticker', label: '', minWidth: 100 },
    {
        id: 'price',
        label: 'Price',
        minWidth: 100,
        align: 'right',
        format: (value: number) => value.toLocaleString('en-US'),
    },
    {
        id: 'sinceOpen',
        label: 'Since open',
        minWidth: 100,
        align: 'center',
    },
    {
        id: 'rsi',
        label: 'RSI',
        minWidth: 80,
        align: 'right',
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
                {columns.map((column) => (
                    <TableCell
                        key={column.id}
                        align={column.align}
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
    return (
        <TableContainer>
            <Table stickyHeader aria-label="sticky table">
                <StockTableHead />
                <TableBody>
                    {stockData.map((row: StockData, idx) =>
                        <StockTableViewRow key={idx} row={row} columns={columns} />
                    )}
                </TableBody>
            </Table>
        </TableContainer>
    );
}

export default StockTableView;