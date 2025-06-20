use atrium_api::{did_doc::DidDocument, types::string::{Did, Handle}};
use hickory_resolver::{config::ResolverConfig, name_server::{GenericConnector, TokioConnectionProvider}, proto::{rr::rdata::TXT, runtime::TokioRuntimeProvider}, Resolver};

use crate::error::{AppError, Result};

pub trait HandleResolver {
    async fn resolve_handle(&self, handle: Handle) -> Option<Did>;
}

pub struct DnsResolver(Resolver<GenericConnector<TokioRuntimeProvider>>);

impl DnsResolver {
    pub fn new() -> DnsResolver {
        let resolver = Resolver::builder_with_config(
            ResolverConfig::cloudflare(),
            TokioConnectionProvider::default(),
        ).build();

        DnsResolver(resolver)
    }
}

impl HandleResolver for DnsResolver {
    async fn resolve_handle(&self, handle: Handle) -> Option<Did> {
        let txt_records = self.0.txt_lookup(
            format!("_atproto.{}", handle.to_string()),
        )
            .await;

        if txt_records.is_err() {
            return None;
        }

        let did_records: Vec<TXT> = txt_records
            .unwrap()
            .into_iter()
            .filter(
                |x| x.to_string().starts_with("did=did")
            )
            .collect();

        if let Some(did_rec) = did_records.first() {
            let did_string = did_rec.to_string();
            let did_str = did_string.split_once('=').unwrap().1;

            if let Ok(did) = Did::new(did_str.to_string()) {
                return Some(did);
            }
        }

        None
    }
}

pub struct WebResolver(reqwest::Client);

impl WebResolver {
    pub fn new() -> Result<WebResolver> {
        let client = reqwest::ClientBuilder::new()
            .build()
            .map_err(|e| AppError::InternalError(e.into()))?;

        Ok(WebResolver(client))
    }
}

impl HandleResolver for WebResolver {
    async fn resolve_handle(&self, handle: Handle) -> Option<Did> {
        let url = format!("https://{}/.well-known/atproto-did", handle.to_string());
        let res = match self.0.get(url).send().await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(?e, "failed web check");
                println!("failed web check: {}", e);
                return None;
            },
        };

        let did_string = match res.text().await {
            Ok(r) => r,
            Err(e) => {
                println!("failed did str parsing: {:?}", e);
                tracing::warn!(?e, "failed did str parsing");
                return None;
            },
        };

        match Did::new(did_string) {
            Ok(did) => Some(did),
            _ => None,
        }
    }
}

pub async fn resolve_handle(handle: Handle) -> Option<Did> {
    let dns_resolver = DnsResolver::new();
    let web_resolver = match WebResolver::new() {
        Ok(r) => r,
        _ => return None,
    };

    if let Some(did) = dns_resolver.resolve_handle(handle.clone()).await {
        return Some(did);
    } else if let Some(did) = web_resolver.resolve_handle(handle).await {
        return Some(did);
    }

    None
}
