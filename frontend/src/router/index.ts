import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import EmployeeTasksView from "../views/EmployeeTasksView.vue";
import AdminDashboardView from "../views/AdminDashboardView.vue";
import RulesManagementView from "../views/RulesManagementView.vue";
import LoginView from "../views/LoginView.vue";
import LoginCallbackView from "../views/LoginCallbackView.vue";

const routes: RouteRecordRaw[] = [
  { path: "/", redirect: "/me/tasks" },
  { path: "/login", component: LoginView },
  { path: "/auth/callback", component: LoginCallbackView },
  { path: "/me/tasks", component: EmployeeTasksView },
  { path: "/admin/dashboard", component: AdminDashboardView },
  { path: "/admin/rules", component: RulesManagementView }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;


