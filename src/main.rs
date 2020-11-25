use ssl_expiration::SslExpiration;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use std::error::Error;
use std::io;
use std::process;
use log::{info, trace, debug};
use std::env;
use prometheus_exporter_base::{render_prometheus, MetricType, PrometheusMetric};

#[derive(Debug, Clone, Default)]
struct MyOptions {}


#[derive(Debug, Deserialize, Default)]
struct Endpoint {
    host: String,
    port: i32,
}

struct Args {
    help: bool,
    verbose: bool,
    endpoints: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = pico_args::Arguments::from_env();

    if (args.contains(["-h", "--help"]))
    {
        println!("usage: \n-h / --help: show this help\n-v / --verbose: enable verbose logging\n--endpoints filename.csv: Use Endpoints from csv (Format: host,port)\n--port: use port to listen to instead of 6661");
        return Ok(());
    }

    let args = Args {
        help: args.contains(["-h", "--help"]),
        verbose: args.contains(["-v", "--verbose"]),
        endpoints: args.value_from_str("--endpoints").unwrap(),
        port: 6661,
    };

    if args.verbose
    {
        env::set_var("RUST_LOG", "trace");
    } else {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();




    let addr = ([0, 0, 0, 0], args.port).into();
    info!("starting exporter on {}", addr);
    let endpoints = args.endpoints.clone();

    render_prometheus(addr, MyOptions::default(), move |request, options| {
        async move {
            trace!(
                "in our render_prometheus(request == {:?}, options == {:?})",
                request,
                options
            );
            let input = File::open(endpoints.as_str()).unwrap();
            let buffered = BufReader::new(input);
            let mut rdr = csv::Reader::from_reader(buffered);
            let expiration_metric = PrometheusMetric::new("tls_expiration", MetricType::Counter, "expiration of tls certificates");
            let mut expiration_buf = expiration_metric.render_header();

            for entry in rdr.deserialize() {
                let record: Endpoint = entry?;
                debug!("endpoint: {:?}:{:?}", record.host, record.port);
                let expiration = SslExpiration::from_addr(format!("{}:{}", record.host, record.port)).unwrap();
                debug!("tls certificate for {}:{} expires in {} days", record.host, record.port, expiration.days());
                let mut attributes: Vec<(&str, &str)> = Vec::new();
                attributes.push(("host", &record.host));

                let mut port_str = format!("{}", record.port);
                attributes.push(("port", &port_str));

                expiration_buf.push_str(&expiration_metric.render_sample(Some(attributes.as_slice()), expiration.days()));
            }


            let mut s = expiration_buf;
            Ok(s)
        }
    })
        .await;
    Ok(())
}
