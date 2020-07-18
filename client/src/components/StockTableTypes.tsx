export interface Column {
  id: "ticker" | "price" | "percentChange" | "rsi";
  numeric: boolean;
  label: string;
  minWidth?: number;
  format?: (value: number) => string;
}


export const columns: Column[] = [
  { id: "ticker", numeric: false, label: "Ticker", minWidth: 50 },
  {
    id: "price",
    numeric: true,
    label: "Price",
    minWidth: 100,
    format: (value: number) => value.toLocaleString("en-US"),
  },
  {
    id: "percentChange",
    numeric: true,
    label: "Since open",
    minWidth: 100,
  },
  {
    id: "rsi",
    numeric: true,
    label: "RSI",
    minWidth: 80,
    format: (value: number) => value.toFixed(2),
  },
];
