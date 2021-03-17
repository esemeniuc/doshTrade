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
    return { name: option.optionType, price: String(option.ask) || "", pop: option.expiration };
}

interface DisplayableOptionQuote {
    name: string,
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
                {displayableOption.name}
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
    return (<TableContainer component={Paper} style={{ height: '100%' }} >
        <Table aria-label="simple table"
            stickyHeader
            style={{
                backgroundColor: 'gainsboro',
                height: '100%'
            }}>
            <TableHead>
                <TableRow>
                    <TableCell align="left">Option</TableCell>
                    <TableCell align="right">Price</TableCell>
                    <TableCell align="right">Probability of Profit</TableCell>
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