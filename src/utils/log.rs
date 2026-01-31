use tracing_subscriber::field::MakeExt;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{EnvFilter, Layer};
use yansi::Paint;

pub struct CompactTime;

impl FormatTime for CompactTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = chrono::Local::now();
        write!(w, "{}", Paint::new(now.format("%H:%M:%S")).dimmed())
    }
}

pub fn default_logging_layer<S>() -> impl Layer<S>
where
    S: tracing::Subscriber,
    S: for<'span> LookupSpan<'span>,
{
    let field_format = tracing_subscriber::fmt::format::debug_fn(|writer, field, value| {
        if field.name() == "message" {
            write!(writer, "{:?}", value)
        } else {
            write!(writer, "{}={:?}", Paint::cyan(field), value)
        }
    })
    .delimited(" ")
    .display_messages();

    tracing_subscriber::fmt::layer()
        .fmt_fields(field_format)
        .with_timer(CompactTime)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .with_test_writer()
}

pub fn json_logging_layer<
    S: for<'a> tracing_subscriber::registry::LookupSpan<'a> + tracing::Subscriber,
>() -> impl tracing_subscriber::Layer<S> {
    Paint::disable();

    tracing_subscriber::fmt::layer()
        .json()
        // Configure the formatter to use `print!` rather than
        // `stdout().write_str(...)`, so that logs are captured by libtest's test
        // capturing.
        .with_test_writer()
}

pub fn filter_layer(level: LogLevel) -> EnvFilter {
    let filter_str = match level {
        LogLevel::Critical => "warn,hyper=off,rustls=off",
        LogLevel::Support => "warn,rocket::support=info,hyper=off,rustls=off",
        LogLevel::Normal => "info,hyper=off,rustls=off",
        LogLevel::Debug => "trace",
        LogLevel::Off => "off",
    };

    tracing_subscriber::filter::EnvFilter::try_new(filter_str).expect("filter string must parse")
}

pub enum LogLevel {
    /// Only shows errors and warnings: `"critical"`.
    Critical,
    /// Shows errors, warnings, and some informational messages that are likely
    /// to be relevant when troubleshooting such as configuration: `"support"`.
    Support,
    /// Shows everything except debug and trace information: `"normal"`.
    Normal,
    /// Shows everything: `"debug"`.
    Debug,
    /// Shows nothing: "`"off"`".
    Off,
}

pub enum LogType {
    Formatted,
    Json,
}

impl From<String> for LogType {
    fn from(input: String) -> Self {
        match input.as_str() {
            "formatted" => Self::Formatted,
            "json" => Self::Json,
            _ => panic!("Unkown log type {}", input),
        }
    }
}

impl From<String> for LogLevel {
    fn from(input: String) -> Self {
        match input.as_str() {
            "critical" => Self::Critical,
            "support" => Self::Support,
            "normal" => Self::Normal,
            "debug" => Self::Debug,
            "off" => Self::Off,
            _ => panic!("Unknown log level {}", input),
        }
    }
}
