pub mod cron_util_2 {
    use std::future::Future;
    use tokio::time::{interval, Duration};

    pub async fn create_cronjob_with_schedule<F, Fut>(sec: u64, job: F)
    where
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut interval = interval(Duration::from_secs(sec));
        loop {
            interval.tick().await;
            job().await;
        }
    }
}
pub mod event_fetcher;
