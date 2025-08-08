use serde::{Deserialize, Serialize};
use crate::error::Result;
use super::{SquareClient, Money};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObject {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub object_type: String,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub version: Option<i64>,
    pub is_deleted: Option<bool>,
    pub catalog_v1_ids: Option<Vec<CatalogV1Id>>,
    pub present_at_all_locations: Option<bool>,
    pub present_at_location_ids: Option<Vec<String>>,
    pub absent_at_location_ids: Option<Vec<String>>,
    pub item_data: Option<CatalogItem>,
    pub category_data: Option<CatalogCategory>,
    pub item_variation_data: Option<CatalogItemVariation>,
    pub tax_data: Option<CatalogTax>,
    pub discount_data: Option<CatalogDiscount>,
    pub modifier_list_data: Option<CatalogModifierList>,
    pub modifier_data: Option<CatalogModifier>,
    pub pricing_rule_data: Option<CatalogPricingRule>,
    pub product_set_data: Option<CatalogProductSet>,
    pub subscription_plan_data: Option<CatalogSubscriptionPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogV1Id {
    pub catalog_v1_id: Option<String>,
    pub location_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub abbreviation: Option<String>,
    pub label_color: Option<String>,
    pub available_online: Option<bool>,
    pub available_for_pickup: Option<bool>,
    pub available_electronically: Option<bool>,
    pub category_id: Option<String>,
    pub tax_ids: Option<Vec<String>>,
    pub modifier_list_info: Option<Vec<CatalogItemModifierListInfo>>,
    pub variations: Option<Vec<CatalogObject>>,
    pub product_type: Option<String>,
    pub skip_modifier_screen: Option<bool>,
    pub item_options: Option<Vec<CatalogItemOptionForItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemModifierListInfo {
    pub modifier_list_id: String,
    pub modifier_overrides: Option<Vec<CatalogModifierOverride>>,
    pub min_selected_modifiers: Option<i32>,
    pub max_selected_modifiers: Option<i32>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModifierOverride {
    pub modifier_id: String,
    pub on_by_default: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemOptionForItem {
    pub item_option_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCategory {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemVariation {
    pub item_id: Option<String>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub upc: Option<String>,
    pub ordinal: Option<i32>,
    pub pricing_type: Option<String>,
    pub price_money: Option<Money>,
    pub location_overrides: Option<Vec<ItemVariationLocationOverrides>>,
    pub track_inventory: Option<bool>,
    pub inventory_alert_type: Option<String>,
    pub inventory_alert_threshold: Option<i64>,
    pub user_data: Option<String>,
    pub service_duration: Option<i64>,
    pub available_for_booking: Option<bool>,
    pub stockable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemVariationLocationOverrides {
    pub location_id: Option<String>,
    pub price_money: Option<Money>,
    pub pricing_type: Option<String>,
    pub track_inventory: Option<bool>,
    pub inventory_alert_type: Option<String>,
    pub inventory_alert_threshold: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogTax {
    pub name: Option<String>,
    pub calculation_phase: Option<String>,
    pub inclusion_type: Option<String>,
    pub percentage: Option<String>,
    pub applies_to_custom_amounts: Option<bool>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogDiscount {
    pub name: Option<String>,
    pub discount_type: Option<String>,
    pub percentage: Option<String>,
    pub amount_money: Option<Money>,
    pub pin_required: Option<bool>,
    pub label_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModifierList {
    pub name: Option<String>,
    pub ordinal: Option<i32>,
    pub selection_type: Option<String>,
    pub modifiers: Option<Vec<CatalogObject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModifier {
    pub name: Option<String>,
    pub price_money: Option<Money>,
    pub ordinal: Option<i32>,
    pub modifier_list_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogPricingRule {
    pub name: Option<String>,
    pub time_period_ids: Option<Vec<String>>,
    pub discount_id: Option<String>,
    pub match_products_id: Option<String>,
    pub apply_products_id: Option<String>,
    pub exclude_products_id: Option<String>,
    pub valid_from_date: Option<String>,
    pub valid_from_local_time: Option<String>,
    pub valid_until_date: Option<String>,
    pub valid_until_local_time: Option<String>,
    pub exclude_strategy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogProductSet {
    pub name: Option<String>,
    pub product_ids_any: Option<Vec<String>>,
    pub product_ids_all: Option<Vec<String>>,
    pub quantity_exact: Option<i64>,
    pub quantity_min: Option<i64>,
    pub quantity_max: Option<i64>,
    pub all_products: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSubscriptionPlan {
    pub name: Option<String>,
    pub phases: Option<Vec<SubscriptionPhase>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPhase {
    pub uid: Option<String>,
    pub cadence: String,
    pub periods: Option<i32>,
    pub recurring_price_money: Option<Money>,
    pub ordinal: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertCatalogObjectRequest {
    pub idempotency_key: String,
    pub object: CatalogObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpsertCatalogObjectsRequest {
    pub idempotency_key: String,
    pub batches: Vec<CatalogObjectBatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectBatch {
    pub objects: Vec<CatalogObject>,
}

impl CatalogObject {
    pub fn upsert(client: &SquareClient, request: &UpsertCatalogObjectRequest) -> Result<Self> {
        client.post("/v2/catalog/object", request)
    }

    pub async fn async_upsert(client: &SquareClient, request: &UpsertCatalogObjectRequest) -> Result<Self> {
        client.async_post("/v2/catalog/object", request).await
    }

    pub fn get(client: &SquareClient, object_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/catalog/object/{}", object_id);
        client.get(&endpoint)
    }

    pub async fn async_get(client: &SquareClient, object_id: &str) -> Result<Self> {
        let endpoint = format!("/v2/catalog/object/{}", object_id);
        client.async_get(&endpoint).await
    }

    pub fn delete(client: &SquareClient, object_id: &str) -> Result<bool> {
        let endpoint = format!("/v2/catalog/object/{}", object_id);
        client.delete(&endpoint)
    }

    pub fn list(client: &SquareClient, types: Option<&str>, cursor: Option<&str>) -> Result<Vec<Self>> {
        let mut endpoint = String::from("/v2/catalog/list?");
        if let Some(t) = types {
            endpoint.push_str(&format!("types={}&", t));
        }
        if let Some(c) = cursor {
            endpoint.push_str(&format!("cursor={}", c));
        }
        client.get(&endpoint)
    }
}