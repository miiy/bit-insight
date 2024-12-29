<template>
    <div class="register-page">
        <h1>Register</h1>
    </div>
</template>

<script lang="ts" setup>
import type { AuthApi } from '@/api'

const req: AuthApi.RegisterRequest = {
    username: 'test',
    password: '123456',
    password_confirmed: '123456',
    email: 'test@test.com',
}

</script>

<style scoped>
</style>