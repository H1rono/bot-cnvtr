use tokio::signal;

#[cfg(unix)]
#[tracing::instrument]
pub async fn signal() {
    use signal::unix;
    let mut terminate = unix::signal(unix::SignalKind::terminate())
        .map_err(|e| {
            tracing::error!("failed to create SIGTERM listener: {}", e);
        })
        .unwrap();
    let mut interrupt = unix::signal(unix::SignalKind::interrupt())
        .map_err(|e| {
            tracing::error!("failed to create SIGINT listener: {}", e);
        })
        .unwrap();
    tokio::select! {
        _ = terminate.recv() => {
            tracing::info!("received SIGTERM");
        }
        _ = interrupt.recv() => {
            tracing::info!("received SIGINT");
        }
    }
}

/// not tested
#[cfg(windows)]
#[tracing::instrument]
pub async fn signal() {
    signal::ctrl_c()
        .await
        .map_err(|e| {
            tracing::error!("failed to listen CTRL-C: {}", e);
        })
        .unwrap();
}
