pub fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}