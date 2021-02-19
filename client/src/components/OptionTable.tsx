import React from 'react';
import styled from 'styled-components'
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Paper from '@material-ui/core/Paper';

function createData(
    name: string,
    price: string,
    pop: string
) {
    return { name, price, pop };
}

const rows = [
    createData("Buy 420 Call", "$2.45", "6%"),
];

const OptionTable = ({ optionQuotes }: any) => {
    console.log("optionQuotes: ", optionQuotes)
    return (<TableContainer component={Paper}>
        <Table aria-label="simple table" style={{ backgroundColor: 'gainsboro' }}>
            <TableHead>
                <TableRow>
                    <TableCell align="left">Option</TableCell>
                    <TableCell align="right">Price</TableCell>
                    <TableCell align="right">Probability of Profit</TableCell>
                </TableRow>
            </TableHead>
            <TableBody>
                {/* {optionQuotes && optionQuotes.map((option: any) => (
                    <TableRow key={option.name} style={{ backgroundColor: 'gray' }}>
                        <TableCell component="th" scope="row">
                            {option.name}
                        </TableCell>
                        <TableCell align="right">{option.price}</TableCell>
                        <TableCell align="right">{option.pop}</TableCell>
                    </TableRow>
                ))} */}
            </TableBody>
        </Table>
    </TableContainer>)
}

export default OptionTable