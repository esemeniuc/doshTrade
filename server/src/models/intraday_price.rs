use diesel::prelude::*;

use crate::models::schema::intraday_prices;
use crate::models::schema::intraday_prices::dsl::*;

#[derive(Identifiable, Queryable, Debug, Clone)]
pub struct IntradayPrice {
	pub id: i32,
	pub stock_id: i32,
	pub price: f64,
	pub volume: i64,
	pub timestamp: chrono::NaiveDateTime,
}

impl IntradayPrice {
	pub fn get_latest(
		conn: &crate::db::DbPoolConn,
		other_stock_id: i32,
	) -> QueryResult<IntradayPrice> {
		intraday_prices
			.filter(stock_id.eq(other_stock_id))
			.order(timestamp.desc())
			.first::<IntradayPrice>(conn)
	}

	fn get_latest_n(
		conn: &crate::db::DbPoolConn,
		other_stock_id: i32,
		n: i32,
	) -> QueryResult<Vec<IntradayPrice>> {
		intraday_prices
			.filter(stock_id.eq(other_stock_id))
			.order(timestamp.desc())
			.limit(n.into())
			.load::<IntradayPrice>(conn)
	}

	pub fn get_rsi(
		conn: &crate::db::DbPoolConn,
		other_stock_id: i32,) -> f32 {
		let rsi_interval = 14;
		let price_structs = intraday_prices
			.filter(stock_id.eq(other_stock_id))
			.order(timestamp.desc())
			.limit(rsi_interval + 1)
			.load::<IntradayPrice>(conn)
			.unwrap().clone();
		let latest_15 = price_structs
			.iter()
			.map(|p| {return p.price})
			.collect::<Vec<f64>>();
		let mut up_price_bars: Vec<f64> = vec!();
		let mut down_price_bars: Vec<f64> = vec!();

		for (i,p) in latest_15.iter().enumerate() {
			if i == rsi_interval as usize {
				break;
			}
			let curr = p;
			let next = latest_15[i+1];
			let price_bar: f64 = next - curr;
			if price_bar < 0.0 {
				down_price_bars.push(price_bar);
			} else {
				up_price_bars.push(price_bar);
			}
		}
		let down_sum: f64 = Iterator::sum(down_price_bars.iter());
		let average_down = f64::abs(f64::from(down_sum) / (down_price_bars.len() as f64));

		let up_sum: f64 = Iterator::sum(up_price_bars.iter());
		let average_up = f64::abs(f64::from(up_sum) / (up_price_bars.len() as f64));

		(f64::from(1) -
			f64::from(1) /
				(f64::from(1) + (average_up / average_down))) as f32
	}

	fn mean(list: &[i32]) -> f64 {
		let sum: i32 = Iterator::sum(list.iter());
		f64::from(sum) / (list.len() as f64)
	}

	pub fn insert(
		conn: &crate::db::DbPoolConn,
		other_stock_ticker: &String,
		other_price: f64,
		other_volume: i64,
		other_timestamp: chrono::NaiveDateTime,
	) -> QueryResult<usize> {
		crate::models::Stock::find(conn, other_stock_ticker).and_then(|stock| {
			diesel::insert_into(intraday_prices::table)
				.values((
					stock_id.eq(stock.id),
					price.eq(other_price),
					volume.eq(other_volume),
					timestamp.eq(other_timestamp),
				))
				.execute(conn)
		})
	}
}
