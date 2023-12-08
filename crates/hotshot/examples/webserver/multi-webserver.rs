use std::sync::Arc;

use async_compatibility_layer::{
    art::async_spawn,
    channel::oneshot,
    logging::{setup_backtrace, setup_logging},
};
use clap::Parser;
use hotshot_testing::state_types::TestTypes;
use surf_disco::Url;
use tracing::error;

#[derive(Parser, Debug)]
struct MultiWebServerArgs {
    consensus_url: Url,
    da_url: Url,
}

#[cfg_attr(async_executor_impl = "tokio", tokio::main)]
#[cfg_attr(async_executor_impl = "async-std", async_std::main)]
async fn main() {
    setup_backtrace();
    setup_logging();

    let args = MultiWebServerArgs::parse();
    let (server_shutdown_sender_cdn, server_shutdown_cdn) = oneshot();
    let (server_shutdown_sender_da, server_shutdown_da) = oneshot();
    let _sender = Arc::new(server_shutdown_sender_cdn);
    let _sender = Arc::new(server_shutdown_sender_da);

    let consensus_server = async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(Some(server_shutdown_cdn), args.consensus_url)
        .await
        {
            error!("Problem starting cdn web server: {:?}", e);
        }
    });
    let da_server = async_spawn(async move {
        if let Err(e) = hotshot_web_server::run_web_server::<
            <TestTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
        >(Some(server_shutdown_da), args.da_url)
        .await
        {
            error!("Problem starting da web server: {:?}", e);
        }
    });

    let _result = futures::future::join_all(vec![consensus_server, da_server]).await;
}