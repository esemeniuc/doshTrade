use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    //the user id
    sub: i32,
    exp: i64,
    nonce: String,
}

#[allow(dead_code)]
pub fn generate_bearer_token_now(user_id: i32) -> String {
    generate_bearer_token(user_id, chrono::Local::now().naive_utc())
}

pub fn generate_bearer_token(user_id: i32, created_at: chrono::NaiveDateTime) -> String {
    let mut rng = thread_rng();
    let chars: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect();

    let my_claims = Claims {
        sub: user_id,
        exp: (created_at + chrono::Duration::weeks(2)).timestamp(),
        nonce: chars,
    };

    encode(
        &Header::new(Algorithm::RS256), //header
        &my_claims,                     //body
        &EncodingKey::from_rsa_pem(PRIV_KEY.as_bytes()).expect("Requires valid PEM"), //secret
    )
        .expect("JWT encode error")
}

/// Validates if a token is not expired, and is signed with decloak private key
/// Note: does not check if a token belongs to a user
#[allow(dead_code)]
pub fn is_valid_token(jwt: &str) -> bool {
    // Adding some leeway (in seconds) for exp and nbf checks
    let validation = Validation {
        leeway: 60,
        algorithms: vec![Algorithm::RS256],
        ..Default::default()
    };
    let token = decode::<Claims>(
        &jwt,
        &DecodingKey::from_rsa_pem(PUB_KEY.as_bytes()).expect("Requires valid PEM"),
        &validation,
    );
    match token {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[allow(dead_code)]
pub fn get_user_id(jwt: &str) -> jsonwebtoken::errors::Result<i32> {
    // Adding some leeway (in seconds) for exp and nbf checks
    let validation = Validation {
        leeway: 60,
        algorithms: vec![Algorithm::RS256],
        ..Default::default()
    };
    let token_data = decode::<Claims>(
        &jwt,
        &DecodingKey::from_rsa_pem(PUB_KEY.as_bytes()).expect("Requires valid PEM"),
        &validation,
    );
    token_data.map(|token| token.claims.sub)
}

const PRIV_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----
MIIEpgIBAAKCAQEAtmeXRaEJeDiows0lOfPKFfmgRHcdc4hJcj3gtAa7E82tGUYq
L/7S5JQ8MT2N0URmvVU2bqdNlXw7pysEtxxEYnySEfe9zvWRfkp3yRcPIBBGxpRg
ekXRjKdHPgqfob1lj6hp/5eGh6IziXpR/GeoWZ2rIuy3ZiOwUGCJ67q0YLsFjsQ1
/tCZflkTcTMormStnNXo3oxZN3nhoBUXWk4Vte6oBDwvszlu+cxfyms2smQacX4L
BketI/r7z9xy5kPsT2VC+V/VQ/jXuNl++TmvTvqTvSCscWvAsOL5XbNobE/BA/1Q
xp80aZdYO48kIRpr7Ijfzp4MWlOvAfJvTXnqQwIDAQABAoIBAQCwQPMkXhKHb8Bq
08437IGGsHqUDcGQpM3LrU0WyDY3UZQFCw6y5/oi07ZCImeKIh1gsBlz3QgT/UKr
Kt9YjODrhdGjuQ9ZD2NydETBNB3ybqflNPTw0cxFrFIj+iH8KMYdhbA/QMzG/Q3t
txvH2OYNClSW38Pr12qEkHRVK0km+Y05IQ8BsAQmxno65nVYYx7G/9Tztt6m08hG
UWKqwXYJyo7Vm9sbAmoF1KHgRP0hGerJr/tMkCCJhcY9LwofgsynrBAsH1VdJ8Do
zDOQRUe8hp/E4GQYOz0pXPy5jcRnSQWZyUmPiMlPDg+PWFBKqpHwpRppL9ulZlgb
4pi1vJqJAoGBAO4DlyNUkNgCV36ecsha55M4B52xuOO//xCC5Ptwa4auzEZe/Vs3
ylo5AIpJVSZy0JAkcWEKuVd19qFFkOEmd4TY/rVSAtII2gARbjfV6PV4FuvitI0n
OxjfldRc34GlP/45JO5fU7Sie/ttPZaYrWweo/nQkokYK61urhqMqXg1AoGBAMQw
Owf5P2edwCEJQgTAdOmNO4W/tQ/Okd/6WM8gJacvVKKoDVvi9eHo05jIN/6jiQe4
r66qa82cPJ5H3Q7ZvEo/2iQ1TD8R+rjpmkIq9NC+u9wqUV+Ap0gJNWom94IQ+y5S
6qSbv8ZDezU7bzr7HMHLnCUnyGVHM4ysTKDAQleXAoGBALCCpSEkY85t3ikepI0F
LVu2X5+I3063YhcCm4IEmxlkpEVDyKxPMZe7UjU7sRQ0bYuYGyUWaOD9sItlJc5z
UouIyUv6p/DPc6w9QPnTf1IbXxRMKg3MHD2UAm9c+LwSquSDyGmm9FKvdTa+isAW
NfZKutxW23UkIbAatq8lxcBJAoGBAJd4gWsjlj+CvT8EbdNLYDid0VHvdHTS0S+z
kvO/5zJQSbEXvNCvx5V29D+mfgz86CRtBD7/1yG1Odwesd7wXv6Yk+yWtdqQIBq6
SV5C2id8y4AtsAvjQcRNT884bKogQD5AyGb01TjXCv5LeBJ6pHY/pLvPHlqLgZJf
yP3n0pDbAoGBAMuxrbHNICu75BsbgjehVbgLi80ilV+doe1OgNneH5nGXCrVnN2t
P5597JDwf6VNGiszc2AEMr3NcLh5wp9J9yTt8zr6hozbgj07aj3EcMpMbeYmGwyl
HgDru/Wd8JjOSYrbBkL2fUcsdMZeSCVZ2yG4IHltulDMoBDb6cvJ5wOw
-----END RSA PRIVATE KEY-----";

const PUB_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAtmeXRaEJeDiows0lOfPK
FfmgRHcdc4hJcj3gtAa7E82tGUYqL/7S5JQ8MT2N0URmvVU2bqdNlXw7pysEtxxE
YnySEfe9zvWRfkp3yRcPIBBGxpRgekXRjKdHPgqfob1lj6hp/5eGh6IziXpR/Geo
WZ2rIuy3ZiOwUGCJ67q0YLsFjsQ1/tCZflkTcTMormStnNXo3oxZN3nhoBUXWk4V
te6oBDwvszlu+cxfyms2smQacX4LBketI/r7z9xy5kPsT2VC+V/VQ/jXuNl++Tmv
TvqTvSCscWvAsOL5XbNobE/BA/1Qxp80aZdYO48kIRpr7Ijfzp4MWlOvAfJvTXnq
QwIDAQAB
-----END PUBLIC KEY-----";

#[test]
fn test_is_valid_token() {
    assert!(is_valid_token(generate_bearer_token_now(1).as_str()))
}

#[test]
fn test_generate_bearer_token() {
    use chrono::{TimeZone};
    let dt = chrono::Utc.ymd(2020, 7, 8).and_hms(9, 10, 11); // `2020-07-08T09:10:11Z`
    assert!(is_valid_token(generate_bearer_token(1, dt.naive_local()).as_str()))
}
