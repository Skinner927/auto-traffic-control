use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use tonic::transport::{Error, Server as GrpcServer};

use atc::v1::event_service_server::EventServiceServer;

use crate::event::EventSender;

use self::event::EventService;

const INTERFACE_VARIABLE: &str = "AUTO_TRAFFIC_CONTROL_INTERFACE";

mod event;

pub struct Api;

impl Api {
    pub async fn serve(event_sender: EventSender) -> Result<(), Error> {
        GrpcServer::builder()
            .add_service(EventServiceServer::new(EventService::new(event_sender)))
            .serve(Self::address_or_default())
            .await
    }

    fn address_or_default() -> SocketAddr {
        if let Ok(address_string) = std::env::var(INTERFACE_VARIABLE) {
            if let Ok(address) = SocketAddr::from_str(&address_string) {
                return address;
            }
        }

        SocketAddr::new(IpAddr::from([0, 0, 0, 0]), 4747)
    }
}

pub trait IntoApi {
    type ApiType;

    fn into_api(self) -> Self::ApiType;
}