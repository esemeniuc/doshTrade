import React from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
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
    align?: 'right' | 'left' | 'center';
    format?: (value: number) => string;
}

export const columns: Column[] = [
    {id: 'ticker', label: '', minWidth: 100},
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

function StickyHeadTable({stockData}: StockTableViewProps) {
    const classes = useStyles();
    return (
        <Paper className={classes.root}>
            <TableContainer>
                <Table stickyHeader aria-label="sticky table">
                    <StockTableHead/>
                    <TableBody>
                        {stockData.map((row) => {
                            return StockTableViewRow(row, columns)
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
        </Paper>
    );
}

export interface StockTableViewProps {
    stockData: StockData[]
}

function StockTableView({stockData}: StockTableViewProps) {
    const classes = useStyles();
    return (
        <div className={classes.root}>
            <Container component="main" maxWidth='sm'>
                <AppBar position="static">
                    <Toolbar>
                        <Typography variant="h6">
                            Yolo Trader
                        </Typography>
                    </Toolbar>
                </AppBar>
                <Typography variant="caption" >
                    <Box textAlign="center" >
                        Since close yesterday
                    </Box>
                </Typography>
                <StickyHeadTable stockData={stockData}></StickyHeadTable>
            </Container>
        </div>
    );
}


export default StockTableView;