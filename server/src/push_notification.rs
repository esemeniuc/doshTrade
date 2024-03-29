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
    // #[derive(RustEmbed)]
    // #[folder = "."]
    // struct Asset;

    let file = std::fs::File::open("private.pem").unwrap();

    let sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info)
        .expect("Failed to generate vapid signature from pem file");
    // sig_builder.add_claim("sub", "mailto:test@example.com");
    // sig_builder.add_claim("foo", "bar");
    // sig_builder.add_claim("omg", 123);
    sig_builder.build()
}

pub fn generate_push_message(
    subscription_info: SubscriptionInfo,
    message: &str,
) -> Result<WebPushMessage, web_push::WebPushError> {
    let mut builder = WebPushMessageBuilder::new(&subscription_info)?;
    let content = message.as_bytes();
    builder.set_vapid_signature(generate_vapid_signature(&subscription_info)?);
    builder.set_payload(ContentEncoding::AesGcm, content);
    builder.build()
}

pub async fn send_demo(message: WebPushMessage) -> Result<(), WebPushError> {
    let client = WebPushClient::new();
    let response = client.send(message).await;
    response
}
