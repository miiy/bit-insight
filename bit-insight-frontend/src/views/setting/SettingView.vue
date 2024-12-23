<template>
  <div class="setting-page">
    <el-form
      ref="ruleFormRef"
      :model="ruleForm"
      status-icon
      :rules="rules"
      label-width="auto"
      class="demo-ruleForm"
    >
      <div class="text-center text-2xl font-bold">Push</div>
      <el-form-item label="push url" prop="push url">
        <el-input v-model="ruleForm.push_url" type="text" disabled />
      </el-form-item>
      <el-form-item label="push_token" prop="push_token">
        <el-input v-model="ruleForm.push_token" type="text" />
      </el-form-item>

      <el-form-item>
        <el-button type="primary" @click="submitForm(ruleFormRef)">
          Save
        </el-button>
      </el-form-item>
    </el-form>
</div>
</template>

<script lang="ts" setup>
import { reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import settingApi from '@/api/setting';

const ruleFormRef = ref<FormInstance>()

const ruleForm = reactive<RuleForm>({
  push_url: 'https://api.test.test/',
  push_token: '',
})

const rules = reactive<FormRules<RuleForm>>({
    push_token: [{ required: true, message: 'Please input the push_token', trigger: 'blur' }],
})

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate((valid, fields) => {
    if (valid) {
      console.log('submit!')
      settingApi.update('push', ruleForm.push_token)
    } else {
      console.log('error submit!')
    }
  })
}

</script>

<style scoped>
.setting-page {
  max-width: 600px;
}
</style>
