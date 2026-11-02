//! OpenTelemetry hook.
//!
//! `init_otel` records the service name and returns whether a collector is
//! configured. It does not start an exporter. When you add `adk-telemetry`,
//! replace the body and keep the signature.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryState {
    pub service_name: String,
    pub exporter_configured: bool,
}

pub fn init_otel(service_name: &str) -> TelemetryState {
    let endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();
    TelemetryState {
        service_name: service_name.to_string(),
        exporter_configured: endpoint.is_some_and(|value| !value.is_empty()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stays_quiet_without_an_endpoint() {
        std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");
        let state = init_otel("wellux");
        assert_eq!(state.service_name, "wellux");
        assert!(!state.exporter_configured);
    }
}
