<template>
  <nav>
    <el-menu
      :default-active="activeIndex"
      class="el-menu-demo"
      mode="horizontal"
      :ellipsis="false"
      @select="handleSelect"
    >
      <el-menu-item index="0">
          <RouterLink to="/">Bit Insight</RouterLink>
      </el-menu-item>
      <el-menu-item index="1" v-if="isAuthenticated"><RouterLink to="/materials">Materials</RouterLink></el-menu-item>
      <el-menu-item index="2" v-if="isAuthenticated"><RouterLink to="/articles">Articles</RouterLink></el-menu-item>
      <el-sub-menu index="3" v-if="isAuthenticated">
        <template #title>{{ user?.username }}</template>
        <el-menu-item><RouterLink to="/profile">Profile</RouterLink></el-menu-item>
        <el-menu-item><RouterLink to="/settings">Setting</RouterLink></el-menu-item>
        <el-menu-item index="2-1">
          <text @click="logout()">logout</text>
        </el-menu-item>
      </el-sub-menu>
      <el-menu-item index="4" v-if="!isAuthenticated">
        <RouterLink to="/login">Login</RouterLink>
      </el-menu-item>
    </el-menu>
  </nav>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import {useAuthStore} from '@/stores'

const router = useRouter()
const authStore = useAuthStore()

const isAuthenticated = computed(() => authStore.isAuthenticated)
const user = computed(() => authStore.user)

const activeIndex = ref('1')
const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

const logout = async () => {
  authStore.logout().then((res) => {
    router.push('/login')
  }).catch((err) => {
    console.log("err", err)
    ElMessage({
      message: err.data.error.message,
      type: 'error',
    })
  })
}
</script>

<style>
.el-menu--horizontal > .el-menu-item:nth-child(1) {
  margin-right: auto;
}
</style>
