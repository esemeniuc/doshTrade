import { StockData } from "../components/StockTableTypes";

function createData(
  ticker: string,
  code: string,
  price: number,
  sinceOpen: number,
  rsi: string
): StockData {
  return { ticker, code, price, sinceOpen, rsi };
}

export const mockStockData = [
  createData("TSLA", "IN", 123.11, 0.5, "0.75"),
  createData("SPY", "CN", 4312.43, 0.4, "0.75"),
  createData("TVIX", "IT", 232.75, 0.6, "0.75"),
  createData("SQ", "US", 11.94, -0.8, "0.15"),
  createData("AAPL", "CA", 424.44, -0.9, "0.75"),
];
