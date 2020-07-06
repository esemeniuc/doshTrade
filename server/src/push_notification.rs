use web_push::*;

#[async_graphql::InputObject]
struct PushSubscriptionKeys {
    p256dh: String,
    auth: String,
}

#[async_graphql::InputObject]
pub(crate) struct PushSubscription {
    endpoint: String,
    expiration_time: Option<String>,
    keys: PushSubscriptionKeys,
}

fn generate_vapid_signature() -> VapidSignature {
    let subscription_info = SubscriptionInfo::new(
        "https://fcm.googleapis.com/fcm/send/fNbXUwtzYcU:APA91bFeBMjyuEy66oeyxtAJDMMtbt6uwyjNXZVNbsGmN2EN5twc9PtHlMNU-5NYVO0qsiGCwtT4Q4G-qU3y77aVEKZ_fEI1SeII3H_rXH5PWExLnXjFM-z9L_LtjYMapsLtveWq6BG0",
        "BNtxN65MgpFzq5VU_fnDe0PDZ6aI9LJHRLkL3Kh66fEW954d8a1xnaeJBzwmAHWt9ldD6V2ajW0GvISVxc4H-i8",
        "h3jhNyqaiptmEATj_5nQrQ",
    );

    let file = std::fs::File::open("private.pem").unwrap();

    let mut sig_builder = VapidSignatureBuilder::from_pem(file, &subscription_info).unwrap();
    sig_builder.add_claim("sub", "mailto:test@example.com");
    sig_builder.add_claim("foo", "bar");
    sig_builder.add_claim("omg", 123);

    return sig_builder.build().unwrap();
}

fn generate_push_message() -> Result<WebPushMessage, web_push::WebPushError> {
    let subscription_info = SubscriptionInfo::new(
        "https://fcm.googleapis.com/fcm/send/fNbXUwtzYcU:APA91bFeBMjyuEy66oeyxtAJDMMtbt6uwyjNXZVNbsGmN2EN5twc9PtHlMNU-5NYVO0qsiGCwtT4Q4G-qU3y77aVEKZ_fEI1SeII3H_rXH5PWExLnXjFM-z9L_LtjYMapsLtveWq6BG0",
        "BNtxN65MgpFzq5VU_fnDe0PDZ6aI9LJHRLkL3Kh66fEW954d8a1xnaeJBzwmAHWt9ldD6V2ajW0GvISVxc4H-i8",
        "h3jhNyqaiptmEATj_5nQrQ",
    );

    let mut builder = WebPushMessageBuilder::new(&subscription_info)?;
    let content = "Encrypted payload to be sent in the notification".as_bytes();
    builder.set_vapid_signature(generate_vapid_signature());
    builder.set_payload(ContentEncoding::AesGcm, content);
    return Ok(builder.build()?);
}

pub async fn send_it() {
    let client = WebPushClient::new();
    let message = generate_push_message().expect("failed to generate push message");
    let response = client.send(message).await;
    response
        .map_err(|e| println!("got error in sendit(), {} ", e))
        .map(|result| println!("Got response: {:?}", result));
}
