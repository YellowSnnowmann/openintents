/** Types for the OpenIntents v1 API. Source of truth: spec/openapi.yaml. */

export type IntentStatus =
  | "queued"
  | "running"
  | "requires_payment"
  | "completed"
  | "failed"
  | "cancelled";

/** Statuses an intent never leaves. */
export const FINAL_STATUSES: readonly IntentStatus[] = ["completed", "failed", "cancelled"];

export interface CreateIntentParams {
  /** What to buy or book, in natural language. */
  intent: string;
  /** Spending cap in cents; the intent fails rather than exceed it. */
  max_amount?: number;
  /** ISO currency code, lowercase. */
  currency?: string;
  /** Structured details: addresses, names, loyalty numbers. */
  context?: Record<string, unknown>;
  /** Always return a payment link instead of paying from the balance. */
  require_payment_link?: boolean;
  /** Up to 20 of your own key/value pairs. */
  metadata?: Record<string, string>;
  /** A customer of your account to buy for: their balance and browser profile. */
  customer?: string;
}

export interface PaymentRequest {
  reason: "insufficient_balance" | "over_limit" | "requested";
  payment_url: string;
  merchant?: string | null;
  total: number;
  fee: number;
  currency: string;
  expires_at: string;
}

export interface ReceiptItem {
  name: string;
  quantity: number;
  amount: number;
}

export interface Receipt {
  merchant: string;
  items?: ReceiptItem[];
  total: number;
  fee: number;
  currency: string;
  confirmation?: string | null;
  receipt_url?: string | null;
}

export interface IntentError {
  code: string;
  message: string;
}

export interface Intent {
  id: string;
  object: "intent";
  status: IntentStatus;
  intent: string;
  /** The customer it buys for; `null` for your account's own intents. */
  customer?: string | null;
  max_amount?: number | null;
  currency?: string | null;
  context?: Record<string, unknown> | null;
  metadata?: Record<string, string> | null;
  /** Set when `status` is `requires_payment`. */
  payment?: PaymentRequest | null;
  /** Set when `status` is `completed`. */
  result?: Receipt | null;
  /** Set when `status` is `failed`. */
  error?: IntentError | null;
  test: boolean;
  created_at: string;
  completed_at?: string | null;
}

export interface ListIntentsParams {
  status?: IntentStatus;
  /** Only intents bought for this customer. */
  customer?: string;
  /** 1 to 100, default 20. */
  limit?: number;
  cursor?: string;
}

export interface IntentList {
  object: "list";
  data: Intent[];
  next_cursor?: string | null;
}

export interface Balance {
  object: "balance";
  /** What intents can pay from now. */
  available: number;
  /** Reserved for intents at their payment step. */
  held?: number;
  currency: string;
  /** The customer whose balance this is; `null` for your account's. */
  customer?: string | null;
  monthly_limit?: number | null;
  spent_this_month?: number;
}

export interface BalanceParams {
  /** A customer's balance instead of your account's. */
  customer?: string;
}

/** Someone you buy for, with their own balance and browser profile. */
export interface Customer {
  id: string;
  object: "customer";
  /** Your own ID for them, unique in your account. */
  external_id?: string | null;
  name?: string | null;
  email?: string | null;
  metadata?: Record<string, string>;
  created_at: string;
}

export interface CustomerParams {
  /** Unique in your account; one already in use is a 409 conflict. */
  external_id?: string;
  name?: string;
  email?: string;
  /** Up to 20 pairs. On update it replaces the whole map. */
  metadata?: Record<string, string>;
}

export interface ListCustomersParams {
  /** Only the customer with this `external_id`. */
  external_id?: string;
  /** 1 to 100, default 20. */
  limit?: number;
  cursor?: string;
}

export interface CustomerList {
  object: "list";
  data: Customer[];
  next_cursor?: string | null;
}

export type TopupStatus = "open" | "paid" | "expired";

export interface CreateTopupParams {
  /** In cents: 50 to 1000000. */
  amount: number;
  /** Only `usd` for now. */
  currency?: string;
  /** Top up this customer's balance instead of your account's. */
  customer?: string;
}

/** A payment link that adds `amount` to a balance once paid. */
export interface Topup {
  id: string;
  object: "topup";
  amount: number;
  currency: string;
  customer?: string | null;
  /** `open` until paid or `expires_at`; a payment begun in time can still turn `expired` into `paid`. */
  status: TopupStatus;
  /** Where the payer pays. */
  url: string;
  expires_at: string;
  created_at: string;
}

export type WebhookEventType =
  | "intent.running"
  | "intent.requires_payment"
  | "intent.completed"
  | "intent.failed"
  | "intent.cancelled";

export interface WebhookEvent {
  id: string;
  type: WebhookEventType;
  created_at: string;
  data: Intent;
}
