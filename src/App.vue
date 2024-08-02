<template>
    <el-config-provider :locale="zhCn">
        <el-container>
            <el-aside>
                <el-menu router :collapse="state.isCollapse" :default-active="route.name" :collapse-transition="true">
                    <template v-for="item in menulist">
                        <el-menu-item :index="item.index" v-if="!item.submenu">
                            <el-icon>
                                <component :is="item.icon" />
                            </el-icon>
                            <template #title>{{ item.title }}</template>
                        </el-menu-item>
                        <el-sub-menu :index="item.index" v-else>
                            <template #title>
                                <el-icon>
                                    <component :is="item.icon" />
                                </el-icon>
                                <span>{{ item.title }}</span>
                            </template>
                            <el-menu-item v-for="subItem in item.submenu" :index="subItem.index">{{ subItem.title }}</el-menu-item>
                        </el-sub-menu>
                    </template>
                </el-menu>
            </el-aside>
            <el-main> <router-view /></el-main>
        </el-container>
    </el-config-provider>
</template>
<script setup>
    import zhCn from "element-plus/dist/locale/zh-cn.mjs";
    import { useDark, useFullscreen } from "@vueuse/core";
    import { useToggle } from "@vueuse/shared";
    import { HomeFilled, Notebook, Document, Headset, Collection, Picture, Tools, Fold, Expand } from "@element-plus/icons-vue";
    const route = useRoute();
    const store = inject("store");
    const toggleThem = () => {
        localStorage.setItem("useDarkKEY", store.userThem);

        const isDark = useDark({
            // 存储到localStorage/sessionStorage中的Key 根据自己的需求更改
            storageKey: "useDarkKEY",
            // 暗黑class名字
            valueDark: "dark",
            // 高亮class名字
            valueLight: "light",
        });
        useToggle(isDark);
    };

    toggleThem();

    const state = reactive({
        isCollapse: true,
        isOpenLogin: false,
    });
    const menulist = [
        { index: "/", icon: HomeFilled, title: "导航" },
        { index: "home2", icon: Notebook, title: "笔记" },
        {
            index: "1",
            icon: Document,
            title: "word工具",
            submenu: [
                { index: "read", title: "word预览" },
                { index: "readExcel", title: "Excel预览" },
                { index: "downExcel", title: "Excel下载" },
            ],
        },
        {
            index: "2",
            icon: Headset,
            title: "音视频工具",
            submenu: [
                { index: "recording", title: "录像" },
                { index: "recordingScreen", title: "录屏" },
                { index: "videoConversion", title: "视频转换器" },
                { index: "audioConversion", title: "音频转换器" },
            ],
        },
        {
            index: "3",
            icon: Picture,
            title: "图片工具",
            submenu: [{ index: "pictureZip", title: "图片压缩" }],
        },
        {
            index: "4",
            icon: Collection,
            title: "收藏资源",
            submenu: [
                { index: "soft", title: "软件" },
                { index: "study", title: "教程" },
                { index: "resources", title: "资源" },
            ],
        },
        {
            index: "5",
            icon: Tools,
            title: "孔大夫的工具箱",
            submenu: [
                { index: "wxgzh", title: "微信公众号" },
                { index: "wxm", title: "微信小程序" },
            ],
        },
    ];
</script>
<style lang="scss" scoped>
    .title {
        width: 100%;
        height: 50px;
        display: flex;
        align-items: center;
        justify-content: center;
        box-sizing: border-box;
        padding: 10px;
        line-height: 50px;
        font-size: 20px;
        font-weight: 600;
        text-overflow: ellipsis;
        white-space: nowrap;
        color: rgb(51, 117, 185);
        position: relative;
        .el-icon {
            cursor: pointer;
        }
    }
</style>
