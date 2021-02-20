import React from 'react';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Paper from '@material-ui/core/Paper';
import { getOptionChain_optionQuote as OptionQuote } from '../graphql/__generated__/getOptionChain'
import { OptionType } from '../graphql/__generated__/globalTypes'

const displayableOptionFrom = (option: OptionQuote): DisplayableOptionQuote => {
    return { name: option.optionType, price: String(option.ask) || "", pop: option.expiration };
}

interface DisplayableOptionQuote {
    name: string,
    price: string,
    pop: string
}

// mocking..
const rows: OptionQuote[] = [{
    __typename: 'OptionQuote',
    optionType: OptionType.CALL,
    expiration: "3/15",
    ask: 113.2,
    delta: 1,
    gamma: 1,
    theta: 1,
    vega: 1,
    rho: 1,
    volatility: 1,
    timeValue: 1,
    strike: 1, bid: 1, last: 1
}]

const OptionRow = ({ option, onClick }: { option: DisplayableOptionQuote, onClick: any }) => {
    return (
        <TableRow style={{ backgroundColor: 'gray' }} onClick={() => { onClick(option) }}>
            <TableCell component="th" scope="row">
                {option.name}
            </TableCell>
            <TableCell align="right">{option.price}</TableCell>
            <TableCell align="right">{option.pop}</TableCell>
        </TableRow>
    )
}

const OptionTable = ({ optionQuotes, onSelectOption }: { optionQuotes: OptionQuote[], onSelectOption: (option: OptionQuote) => void }) => {
    const options = rows;  // optionQuotes when data live
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
                {optionQuotes && options.map((option: OptionQuote, i) =>
                    <OptionRow
                        key={i}
                        option={displayableOptionFrom(option)}
                        onClick={onSelectOption} />)}
            </TableBody>
        </Table>
    </TableContainer>)
}

export default OptionTable