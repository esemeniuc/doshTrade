
export interface Column {
    id: 'ticker' | 'code' | 'price' | 'sinceOpen' | 'rsi';
    numeric: boolean;
    label: string;
    minWidth?: number;
    format?: (value: number) => string;
}

export interface StockData {
    ticker: string;
    code: string;
    price: number;
    sinceOpen: number;
    rsi: string;
}