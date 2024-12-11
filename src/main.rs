//extern crate grpc;
extern crate prost;
extern crate serde_json;
extern crate tonic;

use std::error::Error;

use prost::Message;
use serde_json::Value;
use tonic::transport::Channel;
use tonic::Request;

use opentelemetry::global;
use opentelemetry::sdk::export::metrics::aggregation::cumulative_temporality_selector;
use opentelemetry::sdk::metrics::{self, MeterProvider};
use opentelemetry::KeyValue;

use opentelemetry_proto::collector::metrics::v1::metrics_service_client::MetricsServiceClient;
use opentelemetry_proto::collector::metrics::v1::ExportMetricsServiceRequest;
use opentelemetry_proto::common::v1::{AnyValue, KeyValue as ProtoKeyValue};
use opentelemetry_proto::resource::v1::Resource;

/*
mod opentelemetry {
    tonic::include_proto!("opentelemetry/proto/collector/metrics/v1/metrics_service.grpc.pb");
    tonic::include_proto!("opentelemetry/proto/metrics/v1/metrics.pb");
}
*/

pub struct MetricsClient {
    client: MetricsServiceClient<Channel>,
}

impl MetricsClient {
    pub async fn new(endpoint: &str) -> Result<Self, Box<dyn Error>> {
        let client = MetricsServiceClient::connect(endpoint.to_string()).await?;
        Ok(Self { client })
    }

    pub async fn send_metrics(
        &self,
        request: ExportMetricsServiceRequest,
    ) -> Result<(), Box<dyn Error>> {
        let response = self.client.export(request).await?;
        if response.into_inner().has_next() {
            println!("Metrics sent successfully.");
        } else {
            println!("Failed to send metrics.");
        }
        Ok(())
    }
}
//fn apply_schema(json: &Value, request: &mut ExportMetricsServiceRequest) {
fn apply_schema(request: &mut ExportMetricsServiceRequest) {
    let mut resource_metrics = request.add_resource_metrics();
    resource_metrics.set_schema_url("https://opentelemetry.io/schemas/1.6.1");

    // Add resource attributes
    let mut resource = resource_metrics.mutable_resource();
    let mut attribute = resource.add_attributes();
    attribute.set_key("resource.name".to_string());
    let mut value = attribute.mutable_value();
    value.set_string_value("JTI OT Metrics".to_string());

    let mut scope_metrics = resource_metrics.add_scope_metrics();
    let mut metric = scope_metrics.add_metrics();

    metric.set_name("Interface admin state");
    let mut gauge = metric.mutable_gauge();
    let mut point = gauge.add_data_points();
    let mut key_value = point.add_attributes();
    key_value.set_key("interface");
    let mut value = key_value.mutable_value();
    value.set_string_value("et-0/0/0");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = MetricsClient::new("http://localhost:4317").await?;

    ExportMetricsServiceRequest request;
    apply_schema(request);

    client.SendMetrics(request);

    return 0;
}
