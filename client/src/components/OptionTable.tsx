import React from 'react';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Paper from '@material-ui/core/Paper';
import { getOptionChain_optionQuote as OptionQuote } from '../graphql/__generated__/getOptionChain'

const displayableOptionFrom = (option: OptionQuote): DisplayableOptionQuote => {
    const strike = option.strike ? (Math.round(option.strike * 100) / 100).toFixed(2) : "–"
    const price = option.last ? (Math.round(option.last * 100) / 100).toFixed(2) : "–"
    const pop = option.delta ? (Math.round(option.delta * 100) / 100).toFixed(2) : "–"
    return { strike, price, pop }
}

interface DisplayableOptionQuote {
    strike: string,
    price: string,
    pop: string
}

const OptionRow = ({ option, selected, onClick }:
    {
        option: OptionQuote,
        selected: Boolean,
        onClick: any
    }) => {
    const displayableOption = displayableOptionFrom(option)
    return (
        <TableRow
            style={{ backgroundColor: selected ? 'khaki' : 'linen' }}
            onClick={() => { onClick(option) }}>
            <TableCell component="th" scope="row">
                {displayableOption.strike}
            </TableCell>
            <TableCell align="right">{displayableOption.price}</TableCell>
            <TableCell align="right">{displayableOption.pop}</TableCell>
        </TableRow>
    )
}

const OptionTable = ({ optionQuotes, selectedOption, onSelectOption }:
    {
        optionQuotes: OptionQuote[],
        selectedOption?: OptionQuote,
        onSelectOption: (option: OptionQuote) => void
    }) => {
    const options = optionQuotes;  // optionQuotes when data live
    return (
        <TableContainer component={Paper} style={{ height: '90%' }} >
            <Table aria-label="simple table"
                stickyHeader
                style={{
                    backgroundColor: 'cyan',
                    tableLayout: 'fixed',
                }}>
                <TableHead>
                    <TableRow>
                        <TableCell align="center" style={{ width: 65 }}>Strike</TableCell>
                        <TableCell align="center" style={{ width: 70 }}>Price</TableCell>
                        <TableCell align="center" style={{ width: 100 }}>Probability of Profit</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody >
                    {optionQuotes && options.map((option: OptionQuote, i) =>
                        <OptionRow
                            key={i}
                            option={option}
                            selected={option === selectedOption}
                            onClick={onSelectOption} />)}
                </TableBody>
            </Table>
        </TableContainer>)
}

export default OptionTable