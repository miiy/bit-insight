<template>
  <div class="post-create-page">
    <el-form
      ref="ruleFormRef"
      :model="ruleForm"
      status-icon
      :rules="rules"
      class="demo-ruleForm"
      >
        <el-form-item label="title" prop="title">
            标题：<el-input v-model="ruleForm.title" type="text" />
        </el-form-item>

        <el-form-item label="content" prop="content">
            <Editor />
        </el-form-item>

    </el-form>

    <div>
        Cover：
        <div>
            <el-input v-model="cover" type="text" />
            <el-button type="primary" @click="searchCover(cover)">google search</el-button>
        </div>
        <div>
            <el-button type="primary" @click="generateCover(cover)">generate</el-button>
        </div>
    </div>
  </div>
</template>

<script setup>
import Editor from '@/components/Editor.vue';
import { ref } from 'vue';

const ruleForm = ref({
    title: '',
    content: ''
})

const cover = ref('')

const searchCover = () => {
  // google search image
  const url = `https://www.google.com/search?q=${cover.value}&udm=2`
  window.open(url, '_blank')
}

const generateCover = () => {
    console.log('generateCover')
}
</script>

<style scoped>
.post-create-page {
    /* max-width: 600px; */
}
</style>
