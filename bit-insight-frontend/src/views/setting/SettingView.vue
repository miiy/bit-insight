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
        <el-input v-model="pushUrl" type="text" disabled />
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
import type { SettingApi } from '@/api'

const ruleFormRef = ref<FormInstance>()

const pushUrl = import.meta.env.VITE_API_URL + '/materials/push'

interface RuleForm {
  push_token: string
}

const ruleForm = reactive<RuleForm>({
  push_token: '',
})

const getSetting = async () => {
  const req: SettingApi.GetRequest = {
    key: 'push',
  }
  const res = await settingApi.get(req)
  ruleForm.push_token = res.data.value
}

getSetting()

const rules = reactive<FormRules<RuleForm>>({
    push_token: [{ required: true, message: 'Please input the push_token', trigger: 'blur' }],
})

const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate((valid, fields) => {
    if (valid) {
      const req: SettingApi.UpdateRequest = {
        key: 'push',
        value: ruleForm.push_token,
      }
      settingApi.update(req).then((res) => {
        ElMessage.success('Save success')
      }).catch((err) => {
        ElMessage.error(err.data.error.message)
      })
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
