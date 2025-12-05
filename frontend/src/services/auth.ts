import { ref } from "vue";
import { apiClient } from "./apiClient";

export interface AuthUser {
  id: string;
  email: string;
  display_name: string;
  role: "employee" | "admin";
  compliance_score: number;
}

export interface LoginResponse {
  token: string;
  user: AuthUser;
}

const currentUser = ref<AuthUser | null>(null);
const token = ref<string | null>(null);

export function useAuth() {
  async function loginLocal(email: string, password: string): Promise<void> {
    const resp = await apiClient.post<LoginResponse>("/auth/login", {
      email,
      password
    });
    token.value = resp.token;
    currentUser.value = resp.user;
  }

  async function fetchMe(): Promise<void> {
    const user = await apiClient.get<AuthUser>("/auth/me");
    currentUser.value = user;
  }

  function logout() {
    token.value = null;
    currentUser.value = null;
  }

  return {
    currentUser,
    token,
    loginLocal,
    fetchMe,
    logout
  };
}


