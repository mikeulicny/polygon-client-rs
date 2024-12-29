macro_rules! impl_send_method {
    ($type_name:ty) => {
        pub async fn send(self) -> $type_name {
            self.0.send()
                .await.expect("Send request failed")
                .json::<$type_name>()
                .await.expect("Json parse failed")
        }
    }
}

macro_rules! impl_limit_method {
    () => {
        pub fn limit(mut self, limit: u32) -> Self {
            self.0 = self.0.query(&[("limit", limit)]);
            return self;
        }
    };
}

macro_rules! impl_adjusted_method {
    () => {
        pub fn adjusted(mut self, adjusted: bool) -> Self {
            self.0 = self.0.query(&[("adjusted", adjusted)]);
            return self;
        }
    };
}

macro_rules! impl_order_method {
    () => {
        pub fn order(mut self, order: Order) -> Self {
            self.0 = self.0.query(&[("order", order)]);
            return self;
        }
    };
}

macro_rules! impl_sort_method {
    () => {
        pub fn sort(mut self, sort: Sort) -> Self {
            self.0 = self.0.query(&[("sort", sort)]);
            return self;
        }
    };
}

macro_rules! impl_window_method {
    () => {
        pub fn window(mut self, window: i64) -> Self {
            self.0 = self.0.query(&[("window", window)]);
            return self;
        }
    };
}

macro_rules! impl_ticker_method {
    () => {
        pub fn ticker(mut self, comparator: Comparator, ticker: &str) -> Self {
            self.0 = match comparator {
                Comparator::EQ => self.0.query(&[("ticker", ticker)]),
                Comparator::LT => self.0.query(&[("ticker.lt", ticker)]),
                Comparator::LTE => self.0.query(&[("ticker.lte", ticker)]),
                Comparator::GT => self.0.query(&[("ticker.gt", ticker)]),
                Comparator::GTE => self.0.query(&[("ticker.gte", ticker)]),
            };
            return self;
        }
    };
}

// TODO: Timestamp should take in Nanos, not a string
macro_rules! impl_timestamp_method {
    () => {
        pub fn timestamp(mut self, comparator: Comparator, timestamp: String) -> Self {
            self.0 = match comparator {
                Comparator::EQ => self.0.query(&[("timestamp", timestamp)]),
                Comparator::LT => self.0.query(&[("timestamp.lt", timestamp)]),
                Comparator::LTE => self.0.query(&[("timestamp.lte", timestamp)]),
                Comparator::GT => self.0.query(&[("timestamp.gt", timestamp)]),
                Comparator::GTE => self.0.query(&[("timestamp.gte", timestamp)])
            };
            return self;
        }
    };
}

macro_rules! impl_include_otc_method {
    () => {
        pub fn include_otc(mut self, include_otc: bool) -> Self {
            self.0 = self.0.query(&[("include_otc", include_otc)]);
            return self;
        }
    };
}

macro_rules! impl_asset_class_method {
    () => {
        pub fn asset_class(mut self, asset_class: AssetClass) -> Self {
            self.0 = self.0.query(&[("asset_class", asset_class)]);
            return self;
        }
    };
}

macro_rules! impl_locale_method {
    () => {
        pub fn locale(mut self, locale: MarketLocale) -> Self {
            self.0 = self.0.query(&[("locale", locale)]);
            return self;
        }
    };
}

macro_rules! impl_date_method {
    () => {
        pub fn date(mut self, date: String) -> Self {
            self.0 = self.0.query(&[("date", date)]);
            return self;
        }
    };
}

macro_rules! impl_as_of_method {
    () => {
        pub fn as_of(mut self, date: &'static str) -> Self {
            self.0 = self.0.query(&[("as_of", date)]);
            return self;
        }
    };
}

pub(crate) use {
    impl_send_method,
    impl_limit_method,
    impl_order_method,
    impl_adjusted_method,
    impl_sort_method,
    impl_window_method,
    impl_timestamp_method,
    impl_include_otc_method,
    impl_asset_class_method,
    impl_locale_method,
    impl_ticker_method,
    impl_date_method,
    impl_as_of_method
};
