<template>
  <div class="page">
    <header class="header">
      <div>
        <h2>Dashboard sécurité</h2>
        <p class="subtitle">
          Vue d’ensemble des scores de conformité par équipe.
        </p>
      </div>
    </header>

    <section class="overview">
      <div class="card">
        <p class="label">Score global</p>
        <p class="value">{{ overview?.global_score ?? "–" }}</p>
      </div>
    </section>

    <section class="teams">
      <h3>Équipes</h3>
      <div v-if="loading" class="state">Chargement...</div>
      <div v-else-if="error" class="state error">
        {{ error }}
      </div>
      <div v-else-if="teams.length === 0" class="state">
        Aucune équipe pour l’instant.
      </div>
      <table v-else class="table">
        <thead>
          <tr>
            <th>Équipe</th>
            <th>Score moyen</th>
            <th>Membres</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="t in teams" :key="t.team_id">
            <td>{{ t.team_id }}</td>
            <td>{{ t.average_score }}</td>
            <td>{{ t.users_count }}</td>
          </tr>
        </tbody>
      </table>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { apiClient } from "../services/apiClient";

interface OverviewResponse {
  global_score: number;
}

interface TeamScoreSummary {
  team_id: string;
  average_score: number;
  users_count: number;
}

interface TeamsScoresResponse {
  teams: TeamScoreSummary[];
}

const overview = ref<OverviewResponse | null>(null);
const teams = ref<TeamScoreSummary[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

onMounted(async () => {
  loading.value = true;
  error.value = null;
  try {
    overview.value = await apiClient.get<OverviewResponse>("/scores/overview");
    const resp = await apiClient.get<TeamsScoresResponse>("/scores/teams");
    teams.value = resp.teams;
  } catch (e) {
    console.error(e);
    error.value = "Impossible de charger les scores.";
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

.overview {
  display: flex;
  gap: 1rem;
}

.card {
  padding: 1rem 1.25rem;
  border-radius: 0.75rem;
  background: #020617;
  border: 1px solid #1f2937;
}

.card .label {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
}

.card .value {
  margin: 0.3rem 0 0;
  font-size: 1.4rem;
  font-weight: 700;
}

.teams {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
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


