<template>
  <div class="login-page">
    <el-form
      ref="ruleFormRef"
      :model="ruleForm"
      status-icon
      :rules="rules"
      class="demo-ruleForm"
    >
      <div class="page-title">Login</div>
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
  </div>
</template>

<script lang="ts" setup>
import { reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { useAuthStore } from '@/stores'
import { useRouter } from 'vue-router'
import type { AuthApi } from '@/api'

interface RuleForm {
  username: string
  password: string
}

const authStore = useAuthStore()
const router = useRouter()

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
      const req: AuthApi.LoginRequest = {
        username: ruleForm.username,
        password: ruleForm.password,
      }
      authStore.login(req).then((res) => {
        router.push('/')
      }).catch((err) => {
        ElMessage({
          message: err.data.error.message,
          type: 'error',
        })
      })
    } else {
      console.log('error submit!')
    }
  })
}

</script>

<style scoped>
.login-page {
  width: 100%;
  max-width: 300px;
  margin: 0 auto;
}

.login-page .page-title {
  text-align: center;
  font-size: 24px;
  color: #333;
  margin: 20px 0;
}
</style>
