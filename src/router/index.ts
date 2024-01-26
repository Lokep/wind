import { createRouter, createWebHistory } from "vue-router";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: '/edit'
    },

    {
      path: '/edit',
      name: "home",
      component: () => import("../views/Edit/index.vue"),
    },

    {
      path: '/404',
      name: '404',
      component: () => import("../views/Exception/404.vue")
    },

    {
      path: '/:pathMatch(.*)*',
      redirect: '/404'
    }
  ],

  scrollBehavior() {
    return {
      top: 0,
      left: 0
    }
  }
})