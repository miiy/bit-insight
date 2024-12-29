<template>
    <div>
        hello, {{ profile.username }}
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import profileApi from '@/api/profile'
import type { Profile } from '@/types/profile'

const profile = ref<Profile>({ username: '' })

const getProfile = async () => {
    await profileApi.profile().then((res) => {
        profile.value = res.data
    }).catch((err) => {
        ElMessage.error(err.data.error.message)
    })
}

getProfile()
</script>

<style scoped>
</style>