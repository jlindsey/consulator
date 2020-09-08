use super::{Result, Settings, KV};
use reqwest::Client;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+",
    env!("CARGO_PKG_HOMEPAGE"),
    ")"
);

pub async fn update_consul(settings: &Settings, kvs: Vec<KV>) -> Result<()> {
    let client = Client::builder().user_agent(USER_AGENT).build()?;

    for kv in kvs {
        let mut req = client
            .request(
                reqwest::Method::PUT,
                &format!("{}/v1/kv{}", settings.consul_addr, kv.key),
            )
            .body(kv.value.to_string());
        if let Some(token) = settings.consul_token.as_deref() {
            req = req.bearer_auth(token);
        }
        let req = req.build()?;
        println!("{:?}", req);
        let resp = client.execute(req).await?;
        resp.error_for_status_ref()?;
        println!("{}", resp.text().await?);
    }

    Ok(())
}
