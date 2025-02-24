import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      redirect: "/performance",
    },
    {
      path: "/performance",
      name: "performance",
      component: () => import("@/views/performance/index.vue"),
    },
  ],
});

export default router;
