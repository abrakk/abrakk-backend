import type { Kit, KitListParams } from "../types";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8080";

/**
 * Fetch a paginated, filtered list of kits from the Rust API.
 */
export async function getKits(params: KitListParams = {}): Promise<Kit[]> {
  const query = new URLSearchParams();

  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== "") {
      query.set(key, value);
    }
  });

  const res = await fetch(`${API_URL}/api/v1/kits?${query.toString()}`, {
    next: { revalidate: 60 }, // ISR — revalidate every 60 seconds
  });

  if (!res.ok) {
    throw new Error(`Failed to fetch kits: ${res.status}`);
  }

  return res.json();
}

/**
 * Fetch a single kit by ID.
 */
export async function getKit(id: string): Promise<Kit> {
  const res = await fetch(`${API_URL}/api/v1/kits/${id}`, {
    next: { revalidate: 60 },
  });

  if (res.status === 404) {
    throw new Error("Kit not found");
  }

  if (!res.ok) {
    throw new Error(`Failed to fetch kit: ${res.status}`);
  }

  return res.json();
}

/**
 * Create a new kit. Requires a valid JWT access token.
 */
export async function createKit(
  data: Omit<Kit, "id" | "author_id" | "is_published" | "download_count" | "created_at" | "updated_at">,
  token: string,
): Promise<Kit> {
  const res = await fetch(`${API_URL}/api/v1/kits`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(data),
  });

  if (!res.ok) {
    const error = await res.json().catch(() => ({}));
    throw new Error(error.error ?? `Failed to create kit: ${res.status}`);
  }

  return res.json();
}

/**
 * Record a download for a kit (increments the counter on the backend).
 */
export async function recordKitDownload(id: string): Promise<void> {
  await fetch(`${API_URL}/api/v1/kits/${id}/download`);
}
