use web_push::*;

#[derive(async_graphql::InputObject, Clone, Debug)]
pub struct PushSubscriptionKeys {
    #[graphql(name = "p256dh")]
    pub p256dh: String,
    pub auth: String,
}

#[derive(async_graphql::InputObject, Clone, Debug)]
pub struct PushSubscription {
    pub endpoint: String,
    pub expiration_time: Option<String>,
    pub keys: PushSubscriptionKeys,
}

impl From<PushSubscription> for SubscriptionInfo {
    fn from(push_subscription: PushSubscription) -> Self {
        SubscriptionInfo {
            endpoint: push_subscription.endpoint,
            keys: SubscriptionKeys {
                p256dh: push_subscription.keys.p256dh,
                auth: push_subscription.keys.auth,
            },
        }
    }
}

impl From<crate::models::Client> for SubscriptionInfo {
    fn from(client: crate::models::Client) -> Self {
        SubscriptionInfo {
            endpoint: client.endpoint,
            keys: SubscriptionKeys {
                p256dh: client.p256dh,
                auth: client.auth,
            },
        }
    }
}

fn generate_vapid_signature(
    subscription_info: &web_push::SubscriptionInfo,
) -> Result<VapidSignature, WebPushError> {
    let file = std::fs::File::open("private.pem").unwrap();

    let mut sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info).unwrap();
    sig_builder.add_claim("sub", "mailto:test@example.com");
    sig_builder.add_claim("foo", "bar");
    sig_builder.add_claim("omg", 123);
    sig_builder.build()
}

pub fn generate_push_message(
    subscription_info: SubscriptionInfo,
) -> Result<WebPushMessage, web_push::WebPushError> {
    let mut builder = WebPushMessageBuilder::new(&subscription_info)?;
    let content = "Encrypted payload to be sent in the notification".as_bytes();
    builder.set_vapid_signature(generate_vapid_signature(&subscription_info)?);
    builder.set_payload(ContentEncoding::AesGcm, content);
    builder.build()
}

pub async fn send_demo(message: WebPushMessage) {
    let client = WebPushClient::new();
    let response = client.send(message).await;
    //TODO use something other map
    response
        .map_err(|e| println!("got error in sendit(), {} ", e))
        .map(|result| println!("Got response: {:?}", result));
}
