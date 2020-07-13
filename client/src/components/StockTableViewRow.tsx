import React from "react";
import {
  TableCell,
  Typography,
  TableRow,
  IconButton,
  makeStyles,
  Theme,
  createStyles,
} from "@material-ui/core";
import {
  Notifications,
  NotificationsNone,
  NotificationsOff,
} from "@material-ui/icons";
import { StockData, Column } from "./StockTableTypes";
import { AppContext } from "../redux/context";
import {
  pushPermissionRequest,
  tickerSubscribe,
  tickerUnsubscribe,
} from "../redux/actions";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    mutedButton: {
      color: "gray",
    },
    activeButton: {
      color: "red",
    },
  })
);

function BellButton({ ticker }: { ticker: string }) {
  const {
    state: {
      stockSubscriptionState: { tickers },
      pushState: { userConsent, subscription },
    },
    dispatch,
  } = React.useContext(AppContext);
  const classes = useStyles();
  // TODO: handle this case
  // if (!pushNotificationSupported) {
  //     return (<IconButton style={{ color: 'gray' }} onClick={() => { alert("push not supported") }}><NotificationsOff /></IconButton>)
  // }
  if (userConsent === "default") {
    return (
      <IconButton
        className={classes.mutedButton}
        onClick={() => {
          dispatch(pushPermissionRequest());
        }}
      >
        <NotificationsNone />
      </IconButton>
    );
  }
  if (userConsent === "denied") {
    return (
      <IconButton
        className={classes.mutedButton}
        onClick={() => {
          alert("push permission is denied");
        }}
      >
        <NotificationsOff />
      </IconButton>
    );
  }
  if (!subscription) {
    return (
      <IconButton
        className={classes.activeButton}
        onClick={() => {
          dispatch(pushPermissionRequest());
        }}
      >
        <NotificationsNone />
      </IconButton>
    );
  }
  if (!tickers.includes(ticker)) {
    return (
      <IconButton
        className={classes.activeButton}
        onClick={() => {
          dispatch(tickerSubscribe(ticker));
        }}
      >
        <NotificationsNone />
      </IconButton>
    );
  }

  return (
    <IconButton
      className={classes.activeButton}
      onClick={() => {
        dispatch(tickerUnsubscribe(ticker));
      }}
    >
      <Notifications />
    </IconButton>
  );
}

function RsiCellContent(column: Column, value: string | number) {
  return value;
}

function TickerCellContent(column: Column, value: string | number) {
  return <Typography variant="subtitle2">{value}</Typography>;
}

function PriceCellContent(column: Column, value: string | number) {
  return value;
}

function SinceOpenCellContent(column: Column, value: number) {
  const plusSign = value > 0 ? "+" : "";
  return (
    <Typography variant="subtitle2" color="primary">
      {`${plusSign + value.toFixed(2)}%`}
    </Typography>
  );
}

function StockTableViewRow({
  row,
  columns,
}: {
  row: StockData;
  columns: Column[];
}) {
  return (
    <TableRow hover tabIndex={-1}>
      <TableCell padding="checkbox">
        <BellButton ticker={row.ticker} />
      </TableCell>
      {columns.map((column) => {
        return (
          <TableCell key={column.id} align={column.numeric ? "right" : "left"}>
            {cellContent(row, column)}
          </TableCell>
        );
      })}
    </TableRow>
  );
}

function cellContent(rowData: StockData, column: Column) {
  const value = rowData[column.id];
  switch (column.id) {
    case "rsi":
      return RsiCellContent(column, value);
    case "ticker":
      return TickerCellContent(column, value);
    case "price":
      return PriceCellContent(column, value);
    case "sinceOpen":
      return SinceOpenCellContent(column, value as number);
    default:
      return column.format && typeof value === "number"
        ? column.format(value)
        : value;
  }
}

export default StockTableViewRow;
