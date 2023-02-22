use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;
use minstant::Instant;

use tonic::{Request, Response};
use tonic::transport::Channel;

use super::tracer::Tracer;

pub mod hotel_microservices {
    pub mod geo {
        // The string specified here must match the proto package name
        tonic::include_proto!("geo");
    }
    pub mod rate {
        // The string specified here must match the proto package name
        tonic::include_proto!("rate");
    }
    pub mod search {
        // The string specified here must match the proto package name
        tonic::include_proto!("search");
    }
}

use hotel_microservices::geo::geo_client::GeoClient;
use hotel_microservices::geo::Request as GeoRequest;
use hotel_microservices::rate::rate_client::RateClient;
use hotel_microservices::rate::Request as RateRequest;
use hotel_microservices::search::search_server::Search;
use hotel_microservices::search::{NearbyRequest as SearchRequest, SearchResult};


pub struct SearchService {
    geo_client: Arc<Mutex<GeoClient<Channel>>>,
    rate_client: Arc<Mutex<RateClient<Channel>>>,
    log_path: Option<PathBuf>,
    tracer: RefCell<Tracer>,
}

// This is unsafe
unsafe impl Send for SearchService {}
unsafe impl Sync for SearchService {}

impl Drop for SearchService {
    fn drop(&mut self) {
        let mut tracer = self.tracer.borrow_mut();
        if let Some(path) = &self.log_path {
            if let Some(parent) = path.parent() {
                if let Err(err) = std::fs::create_dir_all(parent) {
                    log::error!("Error create logging dir: {}", err);
                }
            }
            if let Err(err) = tracer.to_csv(path) {
                log::error!("Error writting logs: {}", err);
            }
        }
    }
}

#[tonic::async_trait]
impl Search for SearchService {
    async fn nearby(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResult>, tonic::Status> {
        let request = request.into_inner();
        let result = self
            .nearby_internal(request)
            .await
            .map_err(|err| tonic::Status::internal(err.to_string()))?;
        let resp = Response::new(result);
        Ok(resp)
    }
}

impl SearchService {
    async fn nearby_internal<'s>(&self, request: SearchRequest) -> Result<SearchResult> {
        log::trace!("in Search Nearby");

        log::trace!("nearby lat = {:.4}", request.lat);
        log::trace!("nearby lon = {:.4}", request.lon);
        let geo_req = GeoRequest {
            lat: request.lat,
            lon: request.lon,
        };

        let start = Instant::now();
        let nearby = self.geo_client.lock().await.nearby(geo_req).await?;
        self.tracer
            .borrow_mut()
            .record_end_to_end("geo", start.elapsed())?;
        let nearby = nearby.into_inner();
        log::trace!("get Nearby hotelId = {:?}", nearby.hotel_ids);
        let num_nearby = nearby.hotel_ids.len();
        let rate_req = RateRequest {
            hotel_ids: nearby.hotel_ids,
            in_date: request.in_date,
            out_date: request.out_date,
        };

        let start = Instant::now();
        let rates = self.rate_client.lock().await.get_rates(rate_req).await?;
        self.tracer
            .borrow_mut()
            .record_end_to_end("rate", start.elapsed())?;
        let rates = rates.into_inner();
        let mut hotel_ids = Vec::with_capacity(num_nearby);
        for rate_plan in rates.rate_plans {
            hotel_ids.push(rate_plan.hotel_id);
        }

        let result = SearchResult { hotel_ids };
        Ok(result)
    }
}

impl SearchService {
    pub fn new(geo: GeoClient<Channel>, rate: RateClient<Channel>, log_path: Option<PathBuf>) -> Self {
        let mut tracer = Tracer::new();
        tracer.new_end_to_end_entry("geo");
        tracer.new_end_to_end_entry("rate");
        SearchService {
            geo_client: Arc::new(Mutex::new(geo)),
            rate_client: Arc::new(Mutex::new(rate)),
            log_path,
            tracer: RefCell::new(tracer),
        }
    }
}