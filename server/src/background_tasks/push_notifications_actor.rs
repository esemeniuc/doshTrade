use actix::prelude::*;

pub(crate) struct PushNotificationsActor {
    pub(crate) pool: crate::db::DbPool,
}

impl Actor for PushNotificationsActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let conn = self.pool.to_owned();

        async move {
            let mut interval = actix_web::rt::time::interval(std::time::Duration::from_secs(10));
            loop {
                log::trace!("Sending background push notifications!");
                match send_push_notifications(&conn).await {
                    Ok(_) => log::info!("Completed sending background push notifications"),
                    Err(e) => log::error!("Error sending background push: {}", e)
                };
                interval.tick().await;
            }
        }
            .into_actor(self)
            .spawn(ctx);
    }
}

pub async fn send_push_notifications(
    conn: &crate::db::DbPool,
) -> Result<(), sqlx::Error> {
    let client = web_push::WebPushClient::new();
    #[derive(sqlx::FromRow)]
    struct ClientSubscription {
        id: i32,
        stock_id: i32,
        ticker: String,
        endpoint: String,
        p256dh: String,
        auth: String,
    }
    let client_subs = sqlx::query_as::<_, ClientSubscription>(
        "SELECT client_subscriptions.id, stock_id, ticker, endpoint, p256dh, auth FROM client_subscriptions
        JOIN clients ON clients.id = client_subscriptions.client_id
        JOIN stocks ON stocks.id = client_subscriptions.stock_id
        WHERE last_sent IS NULL OR last_sent < CURRENT_TIMESTAMP - interval '1 hour'")
        .fetch_all(conn)
        .await?;

    let client_subs_fut = client_subs
        .into_iter()
        .map(|sub| async {
            (crate::models::intraday_price::IntradayPrice::get_rsi_by_stock_id(conn, sub.stock_id, 14).await, sub)
        });

    let filtered_client_subs = futures::future::join_all(client_subs_fut)
        .await
        .into_iter()
        .filter_map(|x| match x.0 {
            Ok(val) => Some((val, x.1)),
            Err(_) => None
        })
        .filter_map(|x| {
            let (rsi_val, sub) = x;
            if rsi_val <= 0.15 {
                Some((format!("{} is oversold", sub.ticker), sub))
            } else if rsi_val >= 0.51 {
                Some((format!("{} is overbought", sub.ticker), sub))
            } else {
                return None;
            }
        })
        .collect::<Vec<_>>();

    let ids_to_update = filtered_client_subs.iter().map(|x| x.1.id).collect::<Vec<_>>();
    sqlx::query("UPDATE client_subscriptions
    SET last_sent = CURRENT_TIMESTAMP
    WHERE id IN (SELECT unnest($1::integer[]))")
        .bind(&ids_to_update)
        .execute(conn).await?;

    let messages_to_send = filtered_client_subs.into_iter()
        .filter_map(|x| {
            let (notification_msg, sub) = x;
            let sub_info = web_push::SubscriptionInfo {
                endpoint: sub.endpoint,
                keys: web_push::SubscriptionKeys {
                    p256dh: sub.p256dh,
                    auth: sub.auth,
                },
            };
            crate::push_notification::generate_push_message(sub_info, &notification_msg).ok()
        })
        .map(|msg| client.send(msg));

    //send it!
    let send_results = futures::future::join_all(messages_to_send).await;
    let (_, errs): (Vec<_>, Vec<_>) = itertools::Itertools::partition_map(
        send_results.into_iter(),
        |r| match r {
            Ok(v) => itertools::Either::Left(v),
            Err(v) => itertools::Either::Right(v),
        });
    errs.iter().for_each(|x| log::error!("Failed to send push message: {}", x));
    Ok(())
}
