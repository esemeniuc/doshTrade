import React from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import Container from "@material-ui/core/Container";
import {
    AppBar,
     Chip,
    Paper, Toolbar, Typography
} from "@material-ui/core";

import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TablePagination from '@material-ui/core/TablePagination';
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

interface Column {
    id: 'ticker' | 'code' | 'price' | 'sinceOpen' | 'rsi';
    label: string;
    minWidth?: number;
    align?: 'right' | 'left' | 'center';
    format?: (value: number) => string;
}

const columns: Column[] = [
    { id: 'ticker', label: '', minWidth: 100 },
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

interface Data {
    ticker: string;
    code: string;
    price: number;
    sinceOpen: number;
    rsi: string;
}

function createData(ticker: string, code: string, price: number, sinceOpen: number, rsi: string): Data {
    return { ticker, code, price, sinceOpen, rsi };
}

const rows = [
    createData('TSLA', 'IN', 123.11, 0.5, "0.75"),
    createData('SPY', 'CN', 4312.43, 0.4, "0.75"),
    createData('TVIX', 'IT', 232.75, 0.6, "0.75"),
    createData('SQ', 'US', 11.94, -0.8, "0.15"),
    createData('AAPL', 'CA', 424.44, -0.9, "0.75"),
];

function RsiCellContent(column: Column, value: string | number) {
    return(
        <TableCell key={column.id} align={column.align}>
        <Typography variant="subtitle2">
        {value}
</Typography>
        </TableCell>
        )
}

function TickerCellContent(column: Column, value: string | number) {
    return(
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

function CellContentForDataColumn(rowData: Data, column: Column) {
    const value = rowData[column.id];
    switch (column.id ) {
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
    
function StockTableRow(rowData: Data) {
    return (
        <TableRow hover role="checkbox" tabIndex={-1} key={rowData.code}>
            {columns.map((column) => {
                return CellContentForDataColumn(rowData, column)
            })}
        </TableRow>
    );
}

function StockTableColumnCell(column: Column) {
    return (<TableCell
        key={column.id}
        align={column.align}
        style={{ minWidth: column.minWidth }}
    >
        {column.label}
    </TableCell>)
}

function StickyHeadTable() {
    const classes = useStyles();
    const [page, setPage] = React.useState(0);

    const handleChangePage = (event: unknown, newPage: number) => {
        setPage(newPage);
    };

    return (
        <Paper className={classes.root}>
            <TableContainer>
                <Table stickyHeader aria-label="sticky table">
                    <TableHead>
                        <TableRow>
                            {columns.map((column) => (
                                StockTableColumnCell(column)
                            ))}
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {rows.map((row) => {
                            return StockTableRow(row)                            
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
        </Paper>
    );
}

function StockTableView() {
    const classes = useStyles();
    return (
        <div className={classes.root} >
            <Container component="main" maxWidth='sm'>
                <AppBar position="static">
                    <Toolbar>
                        <Typography variant="h6" >
                            Yolo Trader
                        </Typography>
                    </Toolbar>
                </AppBar>                    
                <StickyHeadTable></StickyHeadTable>
            </Container>
        </div>
    );
}



export default StockTableView;