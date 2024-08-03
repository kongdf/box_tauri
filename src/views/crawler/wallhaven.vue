<template>
    <p @click="test">111</p>
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
    const test=()=>{
        let url='https://w.wallhaven.cc/full/p9/wallhaven-p913ep.jpg'

        fetch(url).then(
            (res) => {
                res.blob().then(
                    (blob) => {
                        console.log(blob)
                        download(blob, '1.jpg');
                    },
                    (err) => {
                        console.log(err);
                    }
                );
            },
            (err) => {
                console.log(err);
            }
        );
    }
    const download = (blob, name) => {
        let href = window.URL.createObjectURL(blob);
        let eleLink = document.createElement("a");
        eleLink.download = name;
        eleLink.href = href;
        eleLink.click();
        eleLink.remove();
        window.URL.revokeObjectURL(href);
    };
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
