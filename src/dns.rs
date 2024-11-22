use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::info;
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, MdnsService, ServiceDiscovery, ServiceRegistration, ServiceType, TxtRecord};

use crate::Result;

/// Options for the scanner
#[derive(Debug, Clone)]
pub struct ScanOptions {
    pub service_port: u16,
    pub hostname: String,
}

#[derive(Default, Clone)]
struct Context {
    service_name: String,
}

/// Scan the network for any other services
/// that are of interest to us.
pub async fn scan_network(opts: ScanOptions) -> Result<()> {
    let sub_types = vec!["poc"];

    let service_type = ServiceType::with_sub_types("cool_stuff", "tcp", sub_types)?;

    let mut service = MdnsService::new(service_type.clone(), opts.service_port);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    txt_record.insert("name", opts.hostname.as_str())?;

    service.set_name(opts.hostname.as_str());
    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));
    service.set_txt_record(txt_record);

    let mut browser = MdnsBrowser::new(service_type);
    browser.set_service_discovered_callback(Box::new(on_service_discovered));

    info!("Starting dns scan");

    let register_loop = service.register()?;
    let browsing_loop = browser.browse_services()?;

    loop {
        register_loop.poll(Duration::from_secs(0))?;
        browsing_loop.poll(Duration::from_secs(0))?;
    }

    Ok(())
}

/// Callback when our service is registered in the mDNS
fn on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    _context: Option<Arc<dyn Any>>
) {
    info!("Service registered: {:?}", result);
}

fn on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>
) {
    info!("Service discovered: {:?}", result);
}