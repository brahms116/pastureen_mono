use shared_models::*;

use reqwest::{Error, Response};

use serde::de::DeserializeOwned;

pub async fn handle_res<T>(res: Result<Response, Error>) -> Result<T, ClientHttpResponseError>
where
    T: DeserializeOwned,
{
    match res {
        Err(err) => Err(ClientHttpResponseError::RawErr(format!("{:?}", err))),
        Ok(res) => {
            handle_reqwest_response::<T>(res).await
        }
    }
}

async fn handle_reqwest_response<T>(res: Response) -> Result<T, ClientHttpResponseError>
where
    T: DeserializeOwned,
{
    let status = res.status();
    if status.is_success() {
        return res
            .json::<T>()
            .await
            .map_err(|err| ClientHttpResponseError::DeserializeErr(format!("{:?}", err)));
    } else {
        return res
            .json::<HttpErrResponseBody>()
            .await
            .map_err(|err| ClientHttpResponseError::DeserializeErr(format!("{:?}", err)))
            .and_then(|err| Err(ClientHttpResponseError::TypedServiceErr(err)));
    }
}
