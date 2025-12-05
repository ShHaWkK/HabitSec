<template>
  <div class="app-shell">
    <header class="app-header">
      <h1>Security UX Layer</h1>
      <nav class="app-nav">
        <RouterLink to="/me/tasks">Mes tâches</RouterLink>
        <RouterLink to="/admin/dashboard">Admin</RouterLink>
      </nav>
    </header>

    <main class="app-main">
      <RouterView />
    </main>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.app-shell {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: #0f172a;
  color: #e5e7eb;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background: #020617;
  border-bottom: 1px solid #1f2937;
}

.app-header h1 {
  font-size: 1.25rem;
}

.app-nav {
  display: flex;
  gap: 1rem;
}

.app-nav a {
  color: #9ca3af;
  text-decoration: none;
}

.app-nav a.router-link-active {
  color: #22c55e;
  font-weight: 600;
}

.app-main {
  flex: 1;
  padding: 1.5rem 2rem;
}
</style>


