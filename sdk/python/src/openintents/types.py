"""Types for the OpenIntents v1 API. Source of truth: spec/openapi.yaml.

Responses are returned as plain dicts typed with TypedDicts, so new fields
from the API never break older SDK versions.
"""

from __future__ import annotations

from typing import Any, Literal, TypedDict

IntentStatus = Literal["queued", "running", "requires_payment", "completed", "failed", "cancelled"]

#: Statuses an intent never leaves.
FINAL_STATUSES: frozenset[str] = frozenset({"completed", "failed", "cancelled"})


class PaymentRequest(TypedDict, total=False):
    reason: Literal["insufficient_balance", "over_limit", "requested"]
    payment_url: str
    merchant: str | None
    total: int
    fee: int
    currency: str
    expires_at: str


class ReceiptItem(TypedDict):
    name: str
    quantity: int
    amount: int


class Receipt(TypedDict, total=False):
    merchant: str
    items: list[ReceiptItem]
    total: int
    fee: int
    currency: str
    confirmation: str | None
    receipt_url: str | None


class IntentError(TypedDict):
    code: str
    message: str


class Intent(TypedDict, total=False):
    id: str
    object: Literal["intent"]
    status: IntentStatus
    intent: str
    customer: str | None
    max_amount: int | None
    currency: str | None
    context: dict[str, Any] | None
    metadata: dict[str, str] | None
    payment: PaymentRequest | None
    result: Receipt | None
    error: IntentError | None
    test: bool
    created_at: str
    completed_at: str | None


class IntentList(TypedDict, total=False):
    object: Literal["list"]
    data: list[Intent]
    next_cursor: str | None


class Balance(TypedDict, total=False):
    object: Literal["balance"]
    available: int
    held: int
    currency: str
    customer: str | None
    monthly_limit: int | None
    spent_this_month: int


class Customer(TypedDict, total=False):
    """Someone you buy for, with their own balance and browser profile."""

    id: str
    object: Literal["customer"]
    external_id: str | None
    name: str | None
    email: str | None
    metadata: dict[str, str]
    created_at: str


class CustomerList(TypedDict, total=False):
    object: Literal["list"]
    data: list[Customer]
    next_cursor: str | None


TopupStatus = Literal["open", "paid", "expired"]


class Topup(TypedDict, total=False):
    """A payment link that adds `amount` to a balance once paid."""

    id: str
    object: Literal["topup"]
    amount: int
    currency: str
    customer: str | None
    status: TopupStatus
    url: str
    expires_at: str
    created_at: str


class WebhookEvent(TypedDict):
    id: str
    type: Literal[
        "intent.running",
        "intent.requires_payment",
        "intent.completed",
        "intent.failed",
        "intent.cancelled",
    ]
    created_at: str
    data: Intent
