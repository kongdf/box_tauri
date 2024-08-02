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

        // let status = await invoke("Crawler_DownImg", {
        //     url,
        // });
        // console.log(status)
    };
    async function greet() {
        state.loading = true;
        state.list = await invoke("Crawler_WallHaven", {
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
