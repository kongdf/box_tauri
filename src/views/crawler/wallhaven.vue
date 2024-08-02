<template>
    <el-scrollbar height="100%" ref="ChatScrollBarRef" v-loading="state.loading">
        <template v-for="(url, index) in state.list" :key="url">
            <el-image style="width: 25%" fit="cover" @click="downIMG(url)" :src="url" lazy />
        </template>
    </el-scrollbar>
</template>

<script setup>
    import { invoke } from "@tauri-apps/api/core";
    const state = reactive({
        list: [],
        loading: false,
        bigList: [],
    });
    const downIMG = async (element) => {
        let parts = element.split("/");
        let url = `https://w.wallhaven.cc/full/${parts[4]}/wallhaven-${parts[5]}`;
            console.log(url)
        let status = await invoke("download_img", {
            url,
        });
        // console.log(status)
    };
    async function greet() {
        state.loading = true;
        state.list = await invoke("greet", {
            page: "1",
        });
        state.bigList = [];
        console.log(state.list);
        state.list.forEach((element) => {
            let parts = element.split("/");
            let url = `https://w.wallhaven.cc/full/${parts[4]}/wallhaven-${parts[5]}`;

            state.bigList.push(url);
        });

        state.loading = false;
    }
    greet();
</script>
