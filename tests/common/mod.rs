use hotstuff::demos::dentry::{Account, Addition, Balance, State, Subtraction, Transaction};

use std::collections::BTreeMap;
use std::env::{var, VarError};
use std::sync::Once;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    prelude::*,
    EnvFilter, Registry,
};

/// Provides a common starting state
pub fn get_starting_state() -> State {
    let balances: BTreeMap<Account, Balance> = vec![
        ("Joe", 1_000_000),
        ("Nathan M", 500_000),
        ("John", 400_000),
        ("Nathan Y", 600_000),
        ("Ian", 0),
    ]
    .into_iter()
    .map(|(x, y)| (x.to_string(), y))
    .collect();
    State { balances }
}

/// Provides a common list of transactions
pub fn get_ten_prebaked_trasnactions() -> Vec<Transaction> {
    vec![
        Transaction {
            add: Addition {
                account: "Ian".to_string(),
                amount: 100,
            },
            sub: Subtraction {
                account: "Joe".to_string(),
                amount: 100,
            },
        },
        Transaction {
            add: Addition {
                account: "John".to_string(),
                amount: 25,
            },
            sub: Subtraction {
                account: "Joe".to_string(),
                amount: 25,
            },
        },
        Transaction {
            add: Addition {
                account: "Nathan Y".to_string(),
                amount: 534044,
            },
            sub: Subtraction {
                account: "Nathan Y".to_string(),
                amount: 534044,
            },
        },
        Transaction {
            add: Addition {
                account: "Nathan Y".to_string(),
                amount: 957954,
            },
            sub: Subtraction {
                account: "Joe".to_string(),
                amount: 957954,
            },
        },
        Transaction {
            add: Addition {
                account: "Nathan M".to_string(),
                amount: 40,
            },
            sub: Subtraction {
                account: "Ian".to_string(),
                amount: 40,
            },
        },
        Transaction {
            add: Addition {
                account: "John".to_string(),
                amount: 404795,
            },
            sub: Subtraction {
                account: "Nathan M".to_string(),
                amount: 404795,
            },
        },
        Transaction {
            add: Addition {
                account: "Joe".to_string(),
                amount: 41312,
            },
            sub: Subtraction {
                account: "Joe".to_string(),
                amount: 41312,
            },
        },
        Transaction {
            add: Addition {
                account: "Joe".to_string(),
                amount: 67763,
            },
            sub: Subtraction {
                account: "Nathan M".to_string(),
                amount: 67763,
            },
        },
        Transaction {
            add: Addition {
                account: "Ian".to_string(),
                amount: 738477,
            },
            sub: Subtraction {
                account: "John".to_string(),
                amount: 738477,
            },
        },
        Transaction {
            add: Addition {
                account: "Joe".to_string(),
                amount: 945443,
            },
            sub: Subtraction {
                account: "Nathan Y".to_string(),
                amount: 945443,
            },
        },
    ]
}

static INIT: Once = Once::new();

pub fn setup_logging() {
    INIT.call_once(|| {
            let internal_event_filter =
                match var("RUST_LOG_SPAN_EVENTS") {
                    Ok(value) => {
                        value
                            .to_ascii_lowercase()
                            .split(",")
                            .map(|filter| match filter.trim() {
                                "new" => FmtSpan::NEW,
                                "enter" => FmtSpan::ENTER,
                                "exit" => FmtSpan::EXIT,
                                "close" => FmtSpan::CLOSE,
                                "active" => FmtSpan::ACTIVE,
                                "full" => FmtSpan::FULL,
                                _ => panic!("test-env-log: RUST_LOG_SPAN_EVENTS must contain filters separated by `,`.\n\t\
                                             For example: `active` or `new,close`\n\t\
                                             Supported filters: new, enter, exit, close, active, full\n\t\
                                             Got: {}", value),
                            })
                            .fold(FmtSpan::NONE, |acc, filter| filter | acc)
                    },
                    Err(VarError::NotUnicode(_)) =>
                        panic!("test-env-log: RUST_LOG_SPAN_EVENTS must contain a valid UTF-8 string"),
                    Err(VarError::NotPresent) => FmtSpan::NONE,
                };
            let fmt_env = var("RUST_LOG_FORMAT").map(|x| x.to_lowercase());
            match fmt_env.as_deref().map(|x| x.trim()) {
                Ok("full") => {
                    let fmt_layer = fmt::Layer::default()
                        .with_span_events(internal_event_filter)
                        .with_ansi(true)
                        .with_test_writer();
                    let _subscriber = Registry::default()
                        .with(EnvFilter::from_default_env())
                        .with(ErrorLayer::default())
                        .with(fmt_layer)
                        .init();
                },
                Ok("json") => {
                    let fmt_layer = fmt::Layer::default()
                        .with_span_events(internal_event_filter)
                        .json()
                        .with_test_writer();
                    let _subscriber = Registry::default()
                        .with(EnvFilter::from_default_env())
                        .with(ErrorLayer::default())
                        .with(fmt_layer)
                        .init();
                },
                Ok("compact") => {
                    let fmt_layer = fmt::Layer::default()
                        .with_span_events(internal_event_filter)
                        .with_ansi(true)
                        .compact()
                        .with_test_writer();
                    let _subscriber = Registry::default()
                        .with(EnvFilter::from_default_env())
                        .with(ErrorLayer::default())
                        .with(fmt_layer)
                        .init();
                },
                _ => {
                    let fmt_layer = fmt::Layer::default()
                        .with_span_events(internal_event_filter)
                        .with_ansi(true)
                        .pretty()
                        .with_test_writer();
                    let _subscriber = Registry::default()
                        .with(EnvFilter::from_default_env())
                        .with(ErrorLayer::default())
                        .with(fmt_layer)
                        .init();
                },
            };
        });
}
