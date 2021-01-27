import React from 'react';
import styled from 'styled-components'
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Paper from '@material-ui/core/Paper';

const GeneratedOption = styled.div`
width: 100%;
margin-top: 20px;
`

const RiskSummaryTable = styled.table`
    width: 100%;
    margin-top: 20px;
    margin-left: 10px;
    margin-right: 10px;
    border-spacing: 0px;
`
const RiskRow = styled.tr`
    height: 40px;
`
const RiskColumnLeft = styled.td`
    border-right: 1px solid black;
    text-align: left;
    width: 40%;
    font-weight: 600;
    font-size: 12px;
`

const RiskColumnRight = styled.td`
    padding-left: 20px;
    text-align: left;
    font-size: 12px;
`


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


const OptionTable = () => {
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
                {rows.map((row) => (
                    <TableRow key={row.name} style={{ backgroundColor: 'gray' }}>
                        <TableCell component="th" scope="row">
                            {row.name}
                        </TableCell>
                        <TableCell align="right">{row.price}</TableCell>
                        <TableCell align="right">{row.pop}</TableCell>
                    </TableRow>
                ))}
            </TableBody>
        </Table>
    </TableContainer>)
}

const RiskSummary = (props: any) => {
    return (
        <RiskSummaryTable>
            <tbody>
                <RiskRow>
                    <RiskColumnLeft>Maximum Risk</RiskColumnLeft>
                    <RiskColumnRight>$1000</RiskColumnRight>
                </RiskRow>
                <RiskRow>
                    <RiskColumnLeft>Maximum Profit</RiskColumnLeft>
                    <RiskColumnRight>$350</RiskColumnRight>
                </RiskRow>
                <RiskRow>
                    <RiskColumnLeft>Break even at expiration</RiskColumnLeft>
                    <RiskColumnRight>$232.35</RiskColumnRight>
                </RiskRow>
            </tbody>
        </RiskSummaryTable>
    )
}


export default function GeneratedResults(props: any) {
    return (
        <GeneratedOption>
            <OptionTable />
            <RiskSummary />
        </GeneratedOption>
    )
}
