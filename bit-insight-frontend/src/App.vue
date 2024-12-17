<script setup>
import { RouterLink, RouterView } from 'vue-router'
import Navbar from './components/layout/Navbar.vue'
import Footer from './components/layout/Footer.vue'
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

</style>
