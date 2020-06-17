import React from 'react';
import {Chip, Typography} from "@material-ui/core";
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';

interface Column {
    id: 'ticker' | 'code' | 'price' | 'sinceOpen' | 'rsi';
    label: string;
    minWidth?: number;
    align?: 'right' | 'left' | 'center';
    format?: (value: number) => string;
}

const columns: Column[] = [
    {
        id: 'ticker',
        label: '',
        minWidth: 100,
    },
    {
        id: 'price',
        label: 'Price',
        minWidth: 120,
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
        minWidth: 120,
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

export interface StockTableViewProps {
    stockData: StockData[]
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

function CellContentForDataColumn(rowData: StockData, column: Column) {
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

function StockTableHead() {
    return (
        <TableHead>
            <TableRow>
                {columns.map((column) => (
                    <TableCell
                        key={column.id}
                        align={column.align}
                        style={{minWidth: column.minWidth}}
                    >
                        {column.label}
                    </TableCell>
                ))}
            </TableRow>
        </TableHead>
    )
}

function StockTableView({stockData}: StockTableViewProps) {
    return (
        <TableContainer>
            <Table stickyHeader aria-label="sticky table">
                <StockTableHead/>
                <TableBody>
                    {stockData.map((row) => {
                        return (
                            <TableRow hover role={"checkbox"} tabIndex={-1} key={row.code}>
                                {columns.map((column) => {
                                    return CellContentForDataColumn(row, column)
                                })}
                            </TableRow>)
                    })}
                </TableBody>
            </Table>
        </TableContainer>
    );
}

export default StockTableView;