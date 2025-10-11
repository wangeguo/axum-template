use tracing_subscriber::EnvFilter;

pub fn setup() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"))
        .add_directive("annota=debug".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
