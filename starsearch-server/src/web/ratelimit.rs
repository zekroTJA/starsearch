use std::time::Duration;

use rocket_governor::{Method, Quota, RocketGovernable};

pub struct Refresh;

impl<'r> RocketGovernable<'r> for Refresh {
    fn quota(_: Method, _: &str) -> Quota {
        Quota::with_period(Duration::from_secs(600))
            .unwrap()
            .allow_burst(Self::nonzero(5))
    }
}
