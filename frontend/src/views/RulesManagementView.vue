<template>
  <div class="page">
    <header class="header">
      <div>
        <h2>Règles de sécurité</h2>
        <p class="subtitle">
          Vue simplifiée des règles actives et de leur impact.
        </p>
      </div>
    </header>

    <section>
      <div v-if="loading" class="state">Chargement des règles...</div>
      <div v-else-if="error" class="state error">
        {{ error }}
      </div>
      <table v-else class="table">
        <thead>
          <tr>
            <th>Clé</th>
            <th>Nom</th>
            <th>Activée</th>
            <th>Utilisateurs impactés</th>
            <th>Taux de complétion</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="r in rules" :key="r.id">
            <td>{{ r.key }}</td>
            <td>{{ r.name }}</td>
            <td>{{ r.enabled ? "Oui" : "Non" }}</td>
            <td>{{ r.impacted_users ?? "–" }}</td>
            <td>
              <span v-if="r.completion_rate !== null && r.completion_rate !== undefined">
                {{ Math.round(r.completion_rate * 100) }} %
              </span>
              <span v-else>–</span>
            </td>
          </tr>
        </tbody>
      </table>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { apiClient } from "../services/apiClient";

interface RuleSummary {
  id: string;
  key: string;
  name: string;
  description: string;
  enabled: boolean;
  impacted_users: number | null;
  completion_rate: number | null;
}

interface RulesListResponse {
  rules: RuleSummary[];
}

const rules = ref<RuleSummary[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

onMounted(async () => {
  loading.value = true;
  error.value = null;
  try {
    const resp = await apiClient.get<RulesListResponse>("/rules");
    rules.value = resp.rules;
  } catch (e) {
    console.error(e);
    error.value = "Impossible de charger les règles.";
  } finally {
    loading.value = false;
  }
});
</script>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.header .subtitle {
  margin: 0;
  color: #9ca3af;
  font-size: 0.9rem;
}

.state {
  padding: 0.75rem 1rem;
  border-radius: 0.75rem;
  background: #020617;
  border: 1px dashed #374151;
  color: #9ca3af;
}

.state.error {
  border-color: #f97373;
  color: #fecaca;
}

.table {
  width: 100%;
  border-collapse: collapse;
  background: #020617;
  border-radius: 0.75rem;
  overflow: hidden;
}

th,
td {
  padding: 0.6rem 0.8rem;
  font-size: 0.85rem;
}

th {
  text-align: left;
  background: #020617;
  border-bottom: 1px solid #1f2937;
  color: #9ca3af;
}

tbody tr:nth-child(even) {
  background: #020617;
}

tbody tr:nth-child(odd) {
  background: #030712;
}
</style>


