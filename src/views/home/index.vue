<template>
    <div>
        首页 
        <span @click="downIMH">下载图片</span>
    </div>
</template>
<script setup>
    import { invoke } from "@tauri-apps/api/core";

const downIMH=async()=>{
    console.log('downIMG')  
    let url ='https://static.kongdf.com/self.jpeg'

    let status = await invoke("download_img", {
            url,
        });
}

</script>