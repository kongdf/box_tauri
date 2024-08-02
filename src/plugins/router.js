import { createRouter, createWebHistory } from "vue-router";


// 路由信息
const routes = [
    {
        path: "/",
        name: "/",
        component: () => import("@/views/home/index.vue"),
    },
    {
        path: "/home2",
        name: "home2",
        component: () => import("@/views/home2/index.vue"),
    },
];

// 导出路由
const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;