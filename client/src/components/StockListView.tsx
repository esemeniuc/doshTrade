import React from 'react';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import Container from "@material-ui/core/Container";
import {
    AppBar,
    Avatar,
    Box, Chip,
    List,
    ListItem,
    ListItemAvatar,
    ListItemSecondaryAction,
    ListItemText,
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
    id: 'name' | 'code' | 'population' | 'size' | 'rsi';
    label: string;
    minWidth?: number;
    align?: 'right';
    format?: (value: number) => string;
}

const columns: Column[] = [
    { id: 'name', label: '$STONK', minWidth: 170 },
    {
        id: 'population',
        label: 'Price',
        minWidth: 70,
        align: 'right',
        format: (value: number) => value.toLocaleString('en-US'),
    },
    {
        id: 'size',
        label: 'Since open',
        minWidth: 70,
        align: 'right',
    },
    {
        id: 'rsi',
        label: 'RSI',
        minWidth: 170,
        align: 'right',
        format: (value: number) => value.toFixed(2),
    },
];

interface Data {
    name: string;
    code: string;
    population: number;
    size: string;
    rsi: string;
}

function createData(name: string, code: string, population: number, size: string, rsi: string): Data {
    return { name, code, population, size, rsi };
}

const rows = [
    createData('TSLA', 'IN', 123.11, "50%", "0.75"),
    createData('SPY', 'CN', 4312.43, "40%", "0.75"),
    createData('TVIX', 'IT', 232.75, "60%", "0.75"),
    createData('SQ', 'US', 11.94, "-80%", "0.15"),
    createData('AAPL', 'CA', 424.44, "-99%", "0.75"),
];


function StickyHeadTable() {
    const classes = useStyles();
    const [page, setPage] = React.useState(0);
    const [rowsPerPage, setRowsPerPage] = React.useState(5);

    const handleChangePage = (event: unknown, newPage: number) => {
        setPage(newPage);
    };

    const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
        setRowsPerPage(+event.target.value);
        setPage(0);
    };

    return (
        <Paper className={classes.root}>
            <TableContainer>
                <Table stickyHeader aria-label="sticky table">
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
                    <TableBody>
                        {rows.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage).map((row) => {
                            return (
                                <TableRow hover role="checkbox" tabIndex={-1} key={row.code}>
                                    {columns.map((column) => {
                                        const value = row[column.id];
                                        if (column.id === 'rsi') {
                                            return (
                                                <TableCell key={column.id} align={column.align}>
                                                    <Chip label={`${value}`} color='primary'/>
                                                </TableCell>
                                            );
                                        } else {
                                            return (
                                                <TableCell key={column.id} align={column.align}>
                                                    {column.format && typeof value === 'number' ? column.format(value) : value}
                                                </TableCell>
                                            );
                                        }
                                    })}
                                </TableRow>
                            );
                        })}
                    </TableBody>
                </Table>
            </TableContainer>
            <TablePagination
                rowsPerPageOptions={[10, 25, 100]}
                component="div"
                count={rows.length}
                rowsPerPage={rowsPerPage}
                page={page}
                onChangePage={handleChangePage}
                onChangeRowsPerPage={handleChangeRowsPerPage}
            />
        </Paper>
    );
}


function StockListView() {
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
                    <Typography variant="caption" >
                        <Box textAlign="center" >
                            Since close yesterday
                        </Box>
                    </Typography>
                    <List className={classes.list}>
                        <ListItem button>
                            <ListItemText id='{1}' primary='TSLA' secondary='Up .86%'/>
                            <ListItemSecondaryAction>
                                <Chip label="1011.87" color='primary'/>
                            </ListItemSecondaryAction>
                        </ListItem>

                        <ListItem button>
                            <ListItemText id='{2}' primary='SQ' secondary='Up 2.51'/>
                            <ListItemSecondaryAction>
                                <Chip label="92.12" color='primary'/>
                            </ListItemSecondaryAction>
                        </ListItem>

                        <ListItem button>
                            <ListItemText id='{3}' primary='SPY' secondary='Up 0.12'/>
                            <ListItemText id='{3}' primary='RSI 0.75' color={'text.secondary'}/>
                            <ListItemSecondaryAction>
                                <Chip label="320.87" color='primary'/>
                            </ListItemSecondaryAction>
                        </ListItem>

                        <ListItem button>
                            <ListItemText id='{3}' primary='TVIX' secondary='Down 5.12%'/>
                            <ListItemSecondaryAction>
                                <Chip label="120.81" color='secondary'/>
                            </ListItemSecondaryAction>
                        </ListItem>


                    </List>
                <StickyHeadTable></StickyHeadTable>
            </Container>
        </div>
    );
}



export default StockListView;