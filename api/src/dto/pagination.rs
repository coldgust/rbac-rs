use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    /// first page: 1
    #[serde(default = "default_page")]
    #[serde_as(as = "DisplayFromStr")]
    pub page: u64,

    #[serde(default = "default_page_size")]
    #[serde_as(as = "DisplayFromStr")]
    pub page_size: u64,

    #[serde(default)]
    pub order_by: Option<String>,

    #[serde(default)]
    pub order_dir: OrderDir,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub enum OrderDir {
    #[default]
    Asc,
    Desc,
}

impl PaginationParams {
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.limit()
    }

    pub fn limit(&self) -> u64 {
        self.page_size.min(200)
    }
}

fn default_page() -> u64 {
    1
}

fn default_page_size() -> u64 {
    10
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub total: Option<u64>,
    pub page: u64,
    pub page_size: u64,
    pub items: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    pub fn new(page: u64, page_size: u64, items: Vec<T>) -> Self {
        Self {
            total: None,
            page,
            page_size,
            items,
        }
    }

    pub fn new_total(total: u64, page: u64, page_size: u64, items: Vec<T>) -> Self {
        Self {
            total: Some(total),
            page,
            page_size,
            items,
        }
    }
}
