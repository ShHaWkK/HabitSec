<template>
  <div class="callback-page">
    <p>Traitement du retour d’authentification...</p>
  </div>
</template>

<script setup lang="ts">
// À implémenter plus tard pour le flux OIDC :
// - lire le code dans l’URL
// - appeler le backend /auth/oidc/callback
// - stocker le token et rediriger
</script>

<style scoped>
.callback-page {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 60vh;
}
</style>


