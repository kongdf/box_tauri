<template>
    <div>
        <div class="settingBox">
            <span>主题：</span>
            <el-switch v-model="store.userThem" active-value="dark" inactive-value="light" size="large" @change="toggleThem" style="--el-switch-on-color: #409eff; --el-switch-off-color: black" inline-prompt :active-icon="Sunny" :inactive-icon="Moon" />
        </div>
    </div>
</template>
<script setup>
    import { Sunny, Moon } from "@element-plus/icons-vue";
    import { useToggle } from "@vueuse/shared";
    import { useDark, useFullscreen } from "@vueuse/core";
    const store = inject("store");
    // const { toggle, isFullscreen } = useFullscreen();

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
</script>
<style lang="scss" scoped>
    .settingBox {
        display: flex;
        align-items: center;
        font-size: 16px;
    }
</style>
