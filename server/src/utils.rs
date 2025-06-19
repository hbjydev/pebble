use atrium_api::types::string::{Did, Handle};
use hickory_resolver::{config::ResolverConfig, name_server::{GenericConnector, TokioConnectionProvider}, proto::{rr::rdata::TXT, runtime::TokioRuntimeProvider}, Resolver};

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
