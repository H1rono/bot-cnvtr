use tokio::signal;

#[cfg(unix)]
#[tracing::instrument]
pub async fn signal() {
    use signal::unix;

    fn inner() -> std::io::Result<(unix::Signal, unix::Signal)> {
        let terminate = unix::signal(unix::SignalKind::terminate()).inspect_err(|e| {
            tracing::error!(error = %e, "Failed to create SIGTERM listener");
        })?;
        let interrupt = unix::signal(unix::SignalKind::interrupt()).inspect_err(|e| {
            tracing::error!(error = %e, "Failed to create SIGINT listener");
        })?;
        std::io::Result::Ok((terminate, interrupt))
    }

    let Ok((mut terminate, mut interrupt)) = inner() else {
        return;
    };
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
    let _ = signal::ctrl_c().await.inspect_err(|e| {
        tracing::error!(error = %e, "Failed to listen CTRL-C");
    });
}
