use shared_models::*;

use reqwest::{Error, Response};

use serde::de::DeserializeOwned;

pub async fn handle_res<T>(res: Result<Response, Error>) -> Result<T, ClientHttpResponseError>
where
    T: DeserializeOwned,
{
    match res {
        Err(err) => Err(ClientHttpResponseError::RawErr(format!("{:?}", err))),
        Ok(res) => handle_reqwest_response::<T>(res).await,
    }
}

async fn handle_reqwest_response<T>(res: Response) -> Result<T, ClientHttpResponseError>
where
    T: DeserializeOwned,
{
    let status = res.status();
    if status.is_success() {
        return res.json::<T>().await.map_err(|err| {
            ClientHttpResponseError::RawErr(format!("Failed to deserialize {:?}", err))
        });
    } else {
        let body_contents = res.bytes().await.map_err(|err| {
            ClientHttpResponseError::RawErr(format!("Failed to get bytes from body: {:?}", err))
        })?;

        serde_json::from_slice::<HttpErrResponseBody>(&body_contents)
            .map_err(|_| {
                ClientHttpResponseError::RawErr(format!(
                    "Status: {:?}\nBody: {}",
                    status,
                    String::from_utf8_lossy(&body_contents)
                ))
            })
            .and_then(|body| Err(ClientHttpResponseError::TypedServiceErr(body)))
    }
}
