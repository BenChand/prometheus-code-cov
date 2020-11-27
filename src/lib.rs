use anyhow::{Error, Result};

// Starting from the current file's state, if this import is removed, code coverage is generated.
use prometheus::Encoder;

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Response, Server,
};

pub fn start_prometheus_and_return_1() -> u32 {
    // Starting from the current file's state, removing the tokio::spawn and start_server() results in code coverage.
    tokio::spawn(start_server());
    1
}

pub async fn start_server() -> Result<()> {
    let addr = ([0, 0, 0, 0], 9000).into();
    let service_fn = make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(|_| async move {
            Response::builder().status(200).body(Body::empty())
        }))
    });
    Server::bind(&addr).serve(service_fn).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_returns_1() {
        assert_eq!(start_prometheus_and_return_1(), 1);
    }
}
