<script lang="ts" setup>
import { RouterView } from 'vue-router'
import Navbar from './views/layout/Navbar.vue'
import Footer from './views/layout/Footer.vue'
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <Navbar />
      </el-header>
      <el-main class="container">
        <RouterView />
      </el-main>
        <Footer />
    </el-container>
  </div>
</template>

<style scoped>
.el-header {
  padding: 0;
}
</style>
