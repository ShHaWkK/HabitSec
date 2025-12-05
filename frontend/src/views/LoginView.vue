<template>
  <div class="login-page">
    <div class="login-card">
      <h2>Connexion</h2>
      <p class="subtitle">
        Mode démo : connecte-toi avec ton email pour tester l’UX sécurité.
      </p>

      <form @submit.prevent="onSubmit" class="login-form">
        <label>
          Email
          <input
            v-model="email"
            type="email"
            required
            placeholder="you@example.com"
          />
        </label>

        <label>
          Mot de passe
          <input
            v-model="password"
            type="password"
            required
            placeholder="********"
          />
        </label>

        <button type="submit" :disabled="loading">
          {{ loading ? "Connexion..." : "Se connecter" }}
        </button>

        <p v-if="error" class="error">
          {{ error }}
        </p>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useAuth } from "../services/auth";

const email = ref("");
const password = ref("");
const loading = ref(false);
const error = ref<string | null>(null);

const router = useRouter();
const { loginLocal } = useAuth();

async function onSubmit() {
  loading.value = true;
  error.value = null;
  try {
    await loginLocal(email.value, password.value);
    router.push("/me/tasks");
  } catch (e) {
    error.value = "Échec de la connexion, vérifie tes identifiants.";
    console.error(e);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.login-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 80vh;
}

.login-card {
  background: #020617;
  border-radius: 0.75rem;
  padding: 2rem;
  max-width: 400px;
  width: 100%;
  border: 1px solid #1f2937;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
}

.login-card h2 {
  margin: 0 0 0.5rem;
}

.subtitle {
  margin: 0 0 1.5rem;
  font-size: 0.9rem;
  color: #9ca3af;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

label {
  display: flex;
  flex-direction: column;
  font-size: 0.9rem;
  gap: 0.35rem;
}

input {
  padding: 0.5rem 0.75rem;
  border-radius: 0.5rem;
  border: 1px solid #374151;
  background: #020617;
  color: #e5e7eb;
}

button {
  margin-top: 0.5rem;
  padding: 0.6rem 1rem;
  border-radius: 999px;
  border: none;
  background: linear-gradient(135deg, #22c55e, #16a34a);
  color: #011806;
  font-weight: 600;
  cursor: pointer;
}

button:disabled {
  opacity: 0.7;
  cursor: default;
}

.error {
  color: #f97373;
  font-size: 0.85rem;
}
</style>


