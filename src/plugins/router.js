import { createRouter, createWebHistory } from "vue-router";

const Crawler = [
    {
        path: "/wallhaven",
        name: "wallhaven",
        component: () => import("@/views/crawler/wallhaven.vue"),
    },
];
// 路由信息
const routes = [
    {
        path: "/",
        name: "/",
        component: () => import("@/views/home/index.vue"),
    },
    {
        path: "/setting",
        name: "setting",
        component: () => import("@/views/setting/index.vue"),
    },
    {
        path: "/setting",
        name: "setting",
        component: () => import("@/views/setting/index.vue"),
    },
    ...Crawler,
];

// 导出路由
const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
