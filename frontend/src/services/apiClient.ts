const API_BASE =
  import.meta.env.VITE_API_BASE_URL?.toString() ?? "/api";

export interface ApiRequestOptions extends RequestInit {
  path: string;
}

async function request<T>(options: ApiRequestOptions): Promise<T> {
  const { path, headers, ...rest } = options;

  const resp = await fetch(`${API_BASE}${path}`, {
    headers: {
      "Content-Type": "application/json",
      ...(headers ?? {})
    },
    ...rest
  });

  if (!resp.ok) {
    const text = await resp.text();
    throw new Error(`API error ${resp.status}: ${text}`);
  }

  if (resp.status === 204) {
    return undefined as unknown as T;
  }

  return (await resp.json()) as T;
}

export const apiClient = {
  get: <T>(path: string) =>
    request<T>({ path, method: "GET" }),
  post: <T>(path: string, body?: unknown) =>
    request<T>({
      path,
      method: "POST",
      body: body ? JSON.stringify(body) : undefined
    }),
  patch: <T>(path: string, body?: unknown) =>
    request<T>({
      path,
      method: "PATCH",
      body: body ? JSON.stringify(body) : undefined
    })
};


