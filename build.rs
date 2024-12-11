use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(true).compile(
        &[
            "proto/opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
            "proto/opentelemetry/proto/metrics/v1/metrics.proto",
            "proto/opentelemetry/proto/resource/v1/resource.proto",
            "proto/opentelemetry/proto/common/v1/common.proto",
        ],
        &["proto"],
    )?;

    Ok(())
}
