<template>
  <el-config-provider :locale="zhCn" size="default">
      <router-view />
  </el-config-provider>
</template>
<script setup>
  import zhCn from "element-plus/es/locale/lang/zh-cn";
  console.info("%c当前服务环境", "color: #409eff;", import.meta.env.VITE_NODE_ENV);
</script>
