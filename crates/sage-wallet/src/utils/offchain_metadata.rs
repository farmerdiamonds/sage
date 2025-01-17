use chia::{protocol::Bytes32, sha2::Sha256};
use sage_assets::{Chip0007Metadata, Collection};
use sage_database::CollectionRow;

#[derive(Debug, Default, Clone)]
pub struct ComputedNftInfo {
    pub name: Option<String>,
    pub sensitive_content: bool,
    pub collection: Option<CollectionRow>,
}

pub fn compute_nft_info(did_id: Option<Bytes32>, blob: Option<&[u8]>) -> ComputedNftInfo {
    let Some(json) = blob.and_then(|blob| Chip0007Metadata::from_bytes(blob).ok()) else {
        return ComputedNftInfo::default();
    };

    let sensitive_content = json.is_sensitive();

    let collection = if let (
        Some(did_id),
        Some(Collection {
            id: metadata_collection_id,
            name,
            attributes,
        }),
    ) = (did_id, json.collection)
    {
        Some(CollectionRow {
            collection_id: calculate_collection_id(did_id, &metadata_collection_id.to_string()),
            did_id,
            metadata_collection_id: metadata_collection_id.to_string(),
            name: Some(name),
            icon: attributes.unwrap_or_default().into_iter().find_map(|item| {
                match (item.kind.as_str(), item.value.as_str()) {
                    (Some("icon"), Some(icon)) => Some(icon.to_string()),
                    _ => None,
                }
            }),
            visible: true,
        })
    } else {
        None
    };

    ComputedNftInfo {
        name: Some(json.name.clone()),
        sensitive_content,
        collection,
    }
}

fn calculate_collection_id(did_id: Bytes32, json_collection_id: &str) -> Bytes32 {
    let mut hasher = Sha256::new();
    hasher.update(did_id);
    hasher.update(json_collection_id);
    hasher.finalize().into()
}
