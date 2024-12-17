<template>
  <el-form
    ref="ruleFormRef"
    style="max-width: 600px"
    :model="ruleForm"
    status-icon
    :rules="rules"
    label-width="auto"
    class="demo-ruleForm"
  >
    <div class="text-center text-2xl font-bold">Login</div>
    <el-form-item label="username" prop="username">
      <el-input v-model="ruleForm.username" type="text" />
    </el-form-item>

    <el-form-item label="password" prop="password">
      <el-input v-model="ruleForm.password" type="password" />
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="submitForm(ruleFormRef)">
        Login
      </el-button>
    </el-form-item>
  </el-form>
</template>

<script lang="ts" setup>
import { reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { useAuthStore } from '@/stores'

const authStore = useAuthStore()

const ruleFormRef = ref<FormInstance>()

const ruleForm = reactive<RuleForm>({
  password: '',
  username: '',
})

const rules = reactive<FormRules<RuleForm>>({
    username: [{ required: true, message: 'Please input the username', trigger: 'blur' }],
    password: [{ required: true, message: 'Please input the password', trigger: 'blur' }],
})

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log('submit!')
      authStore.login(ruleForm)
    } else {
      console.log('error submit!')
    }
  })
}

</script>

<style scoped>
</style>