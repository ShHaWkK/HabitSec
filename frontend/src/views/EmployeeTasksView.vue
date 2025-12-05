<template>
  <div class="page">
    <section class="header">
      <div>
        <h2>Mes tâches sécurité</h2>
        <p class="subtitle">
          Complète ces missions pour faire monter ton score de conformité.
        </p>
      </div>
      <div class="score" v-if="user">
        <span class="label">Score</span>
        <span class="value">{{ user.compliance_score }}</span>
      </div>
    </section>

    <section class="tasks">
      <div v-if="loading" class="state">Chargement des tâches...</div>
      <div v-else-if="error" class="state error">{{ error }}</div>
      <div v-else-if="tasks.length === 0" class="state">
        Tu es à jour, aucune tâche en attente 🎉
      </div>

      <div v-else class="task-list">
        <article
          v-for="task in tasks"
          :key="task.id"
          class="task-card"
        >
          <div class="task-header">
            <h3>{{ task.title }}</h3>
            <span class="badge" :data-status="task.status">
              {{ statusLabel(task.status) }}
            </span>
          </div>
          <p class="description">
            {{ task.description }}
          </p>

          <div class="meta">
            <span class="points">+{{ task.points }} pts</span>
            <span
              v-if="task.due_date"
              class="due"
            >
              À faire avant {{ formatDate(task.due_date) }}
            </span>
          </div>

          <div class="actions">
            <button
              v-if="task.status !== 'done'"
              @click="markDone(task.id)"
            >
              Marquer comme fait
            </button>
            <span v-else class="done-label">Complétée ✅</span>
          </div>
        </article>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, computed } from "vue";
import { apiClient } from "../services/apiClient";
import { useAuth } from "../services/auth";

type TaskStatus = "todo" | "inprogress" | "done";
type TaskType = "mfa" | "device" | "email" | "adminhygiene" | "other";

interface Task {
  id: string;
  user_id: string;
  task_type: TaskType;
  title: string;
  description: string;
  status: TaskStatus;
  points: number;
  due_date: string | null;
  created_at: string;
  updated_at: string;
}

interface TasksListResponse {
  tasks: Task[];
}

const tasks = ref<Task[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

const { currentUser, fetchMe } = useAuth();
const user = computed(() => currentUser.value);

onMounted(async () => {
  loading.value = true;
  error.value = null;
  try {
    await fetchMe();
    const resp = await apiClient.get<TasksListResponse>("/tasks/me");
    tasks.value = resp.tasks;
  } catch (e) {
    console.error(e);
    error.value = "Impossible de charger tes tâches pour le moment.";
  } finally {
    loading.value = false;
  }
});

function statusLabel(status: TaskStatus): string {
  switch (status) {
    case "todo":
      return "À faire";
    case "inprogress":
      return "En cours";
    case "done":
      return "Fait";
  }
}

function formatDate(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleDateString("fr-FR", {
    day: "2-digit",
    month: "short"
  });
}

async function markDone(id: string) {
  try {
    await apiClient.patch<unknown>(`/tasks/${id}/status`, {
      status: "done"
    });
    tasks.value = tasks.value.map((t) =>
      t.id === id ? { ...t, status: "done" } : t
    );
  } catch (e) {
    console.error(e);
    error.value = "Impossible de mettre à jour la tâche.";
  }
}
</script>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.subtitle {
  margin: 0;
  color: #9ca3af;
  font-size: 0.9rem;
}

.score {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  padding: 0.75rem 1rem;
  border-radius: 0.75rem;
  background: radial-gradient(circle at top, #22c55e33, transparent 70%);
  border: 1px solid #16a34a;
}

.score .label {
  font-size: 0.75rem;
  color: #9ca3af;
}

.score .value {
  font-size: 1.5rem;
  font-weight: 700;
}

.tasks {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.state {
  padding: 1rem;
  border-radius: 0.75rem;
  background: #020617;
  border: 1px dashed #374151;
  color: #9ca3af;
}

.state.error {
  border-color: #f97373;
  color: #fecaca;
}

.task-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 1rem;
}

.task-card {
  background: #020617;
  border-radius: 0.75rem;
  padding: 1rem;
  border: 1px solid #1f2937;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1rem;
}

.task-header h3 {
  margin: 0;
  font-size: 1rem;
}

.badge {
  font-size: 0.7rem;
  text-transform: uppercase;
  padding: 0.25rem 0.5rem;
  border-radius: 999px;
  border: 1px solid #374151;
}

.badge[data-status="todo"] {
  border-color: #fbbf24;
  color: #facc15;
}

.badge[data-status="inprogress"] {
  border-color: #38bdf8;
  color: #7dd3fc;
}

.badge[data-status="done"] {
  border-color: #22c55e;
  color: #bbf7d0;
}

.description {
  margin: 0;
  color: #d1d5db;
  font-size: 0.9rem;
}

.meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.8rem;
  color: #9ca3af;
}

.points {
  color: #22c55e;
  font-weight: 600;
}

.actions {
  margin-top: 0.5rem;
  display: flex;
  justify-content: flex-end;
}

button {
  border-radius: 999px;
  border: none;
  padding: 0.4rem 0.9rem;
  background: #22c55e;
  color: #022c12;
  font-size: 0.85rem;
  cursor: pointer;
}

.done-label {
  font-size: 0.85rem;
  color: #22c55e;
}
</style>


