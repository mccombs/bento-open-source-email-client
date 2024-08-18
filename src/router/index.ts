import { RouteRecordRaw } from "vue-router";
import Login from "../views/auth/Index.vue";
import Inbox from "../views/inbox/Index.vue";
import Onboarding from "../views/onboarding/Index.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "Login",
    component: Login,
    // beforeEnter: (to, from, next) => {
    //     useUserStore()
    //         .fetchUser()
    //         .then(() => {
    //             if (useUserStore().user) {
    //                 next("/inbox");
    //             } else {
    //                 next();
    //             }
    //         });
    // },
  },

  {
    path: "/onboarding",
    name: "Onboarding",
    component: Onboarding,
  },
  {
    path: "/inbox",
    name: "Inbox",
    component: Inbox,
  },
];

import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
