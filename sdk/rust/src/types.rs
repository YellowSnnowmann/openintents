//! Types for the OpenIntents v1 API. Source of truth: `spec/openapi.yaml`.
//! Unknown fields are ignored and unknown enum values map to `Unknown`, so new
//! API fields never break older SDK versions.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentStatus {
    Queued,
    Running,
    RequiresPayment,
    Completed,
    Failed,
    Cancelled,
    #[serde(other)]
    Unknown,
}

impl IntentStatus {
    /// `completed`, `failed` and `cancelled` are final.
    pub fn is_final(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::RequiresPayment => "requires_payment",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
        }
    }
}

/// Parameters for `POST /intents`. Build with [`CreateIntent::new`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateIntent {
    /// What to buy or book, in natural language.
    pub intent: String,
    /// Spending cap in cents; the intent fails rather than exceed it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Structured details: addresses, names, loyalty numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Value>,
    /// Always return a payment link instead of paying from the balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_payment_link: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// A customer of your account to buy for: their balance and browser profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
}

impl CreateIntent {
    pub fn new(intent: impl Into<String>) -> Self {
        Self {
            intent: intent.into(),
            ..Default::default()
        }
    }

    pub fn max_amount(mut self, cents: i64) -> Self {
        self.max_amount = Some(cents);
        self
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn context(mut self, context: Value) -> Self {
        self.context = Some(context);
        self
    }

    pub fn require_payment_link(mut self, yes: bool) -> Self {
        self.require_payment_link = Some(yes);
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.customer = Some(customer.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub reason: String,
    pub payment_url: String,
    #[serde(default)]
    pub merchant: Option<String>,
    pub total: i64,
    pub fee: i64,
    pub currency: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptItem {
    pub name: String,
    pub quantity: i64,
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub merchant: String,
    #[serde(default)]
    pub items: Vec<ReceiptItem>,
    pub total: i64,
    pub fee: i64,
    pub currency: String,
    #[serde(default)]
    pub confirmation: Option<String>,
    #[serde(default)]
    pub receipt_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub id: String,
    pub status: IntentStatus,
    pub intent: String,
    /// The customer it buys for; `None` for your account's own intents.
    #[serde(default)]
    pub customer: Option<String>,
    #[serde(default)]
    pub max_amount: Option<i64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub context: Option<Value>,
    #[serde(default)]
    pub metadata: Option<HashMap<String, String>>,
    /// Set when `status` is `requires_payment`.
    #[serde(default)]
    pub payment: Option<PaymentRequest>,
    /// Set when `status` is `completed`.
    #[serde(default)]
    pub result: Option<Receipt>,
    /// Set when `status` is `failed`.
    #[serde(default)]
    pub error: Option<IntentError>,
    #[serde(default)]
    pub test: bool,
    pub created_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListIntents {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IntentStatus>,
    /// Only intents bought for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentList {
    pub data: Vec<Intent>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// What intents can pay from now.
    pub available: i64,
    /// Reserved for intents at their payment step.
    #[serde(default)]
    pub held: Option<i64>,
    pub currency: String,
    /// The customer whose balance this is; `None` for your account's.
    #[serde(default)]
    pub customer: Option<String>,
    #[serde(default)]
    pub monthly_limit: Option<i64>,
    #[serde(default)]
    pub spent_this_month: Option<i64>,
}

/// Someone you buy for, with their own balance and browser profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: String,
    /// Your own ID for them, unique in your account.
    #[serde(default)]
    pub external_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    pub created_at: String,
}

/// Fields for creating or updating a customer. On update, fields left out
/// stay as they are, an empty string clears `external_id`, `name` or
/// `email`, and `metadata` replaces the whole map.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CustomerParams {
    /// Unique in your account; one already in use is a 409 conflict.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl CustomerParams {
    pub fn external_id(mut self, id: impl Into<String>) -> Self {
        self.external_id = Some(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListCustomers {
    /// Only the customer with this `external_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerList {
    pub data: Vec<Customer>,
    #[serde(default)]
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopupStatus {
    /// Waiting to be paid until `expires_at`.
    Open,
    Paid,
    /// Not paid in time (a payment begun in time can still turn it `Paid`).
    Expired,
    #[serde(other)]
    Unknown,
}

/// Parameters for `POST /topups`. Build with [`CreateTopup::new`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateTopup {
    /// In cents: 50 to 1000000.
    pub amount: i64,
    /// Only `usd` for now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Top up this customer's balance instead of your account's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
}

impl CreateTopup {
    pub fn new(amount: i64) -> Self {
        Self {
            amount,
            ..Default::default()
        }
    }

    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.customer = Some(customer.into());
        self
    }
}

/// A payment link that adds `amount` to a balance once paid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topup {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    #[serde(default)]
    pub customer: Option<String>,
    pub status: TopupStatus,
    /// Where the payer pays.
    pub url: String,
    pub expires_at: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub created_at: String,
    pub data: Intent,
}
