export type Fetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export const api_endpoint: string = import.meta.env.VITE_API_ENDPOINT;