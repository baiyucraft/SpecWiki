import axios from "axios";

export const apiBase = import.meta.env.VITE_API_BASE || "http://127.0.0.1:4096/api";

export const apiClient = axios.create({
  baseURL: apiBase,
});
