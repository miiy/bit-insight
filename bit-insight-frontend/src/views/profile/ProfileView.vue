<template>
    <div>
        hello, {{ profile.name }}
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import profileApi from '@/api/profile'

const profile = ref({})

const getProfile = async () => {
    const resp = await profileApi.profile()
    console.log(resp)
}

getProfile()
</script>

<style scoped>
</style>