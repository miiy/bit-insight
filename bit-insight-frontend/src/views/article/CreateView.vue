<template>
  <div class="article-create-page">
    <el-form
      ref="ruleFormRef"
      :model="ruleForm"
      status-icon
      :rules="rules"
      class="demo-ruleForm"
      >

      <el-row>
        <el-form-item prop="content">
            <ArticleEditor v-model="ruleForm.content" />
        </el-form-item>
      </el-row>
      <el-row>
        <!-- submit -->
        <el-form-item>
          <el-button type="primary" @click="submitForm">Submit</el-button>
        </el-form-item>
      </el-row>
    </el-form>
    <div>
      Cover：
      <el-row>
        <el-col :span="12">
          <el-form-item label="keywords" prop="keywords">
              <el-input v-model="cover" type="text" />
            </el-form-item>
        </el-col>
      </el-row>
      <el-form-item>
        <el-button type="primary" @click="searchCover(cover)">google search</el-button>
        <el-button type="primary" @click="generateCover(cover)">generate</el-button>
      </el-form-item>
    </div>
  </div>
</template>

<script lang="ts" setup>
import ArticleEditor from '@/components/ArticleEditor.vue';
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

const submitForm = () => {
    console.log('submitForm')
    console.log(ruleForm.value)
}
</script>

<style scoped>
.article-create-page {
    /* max-width: 600px; */
}
</style>
