<template>
    <div class="article-detail-page">
        <el-row>
            <el-col :span="24">
                <!-- {"id":100,"category_id":1,"title":"title50","author":"admin50","source":"","source_url":"","thumbnail":"","summary":"","content":"content50","created_at":"+002024-12-23T11:50:42.000000000Z","updated_at":"+002024-12-23T11:50:42.000000000Z"} -->
                <h1>{{ article.title }}</h1>
                <div>
                    <span><el-tag type="primary">{{ article.category_id }}</el-tag></span>
                    <el-divider direction="vertical" />
                    <span>{{ article.author }}</span>
                    <el-divider direction="vertical" />
                    <span><a href="{{ article.source_url }}">{{ article.source }}</a></span>
                    <el-divider direction="vertical" />
                    <span>{{ article.created_at }}</span>
                </div>
                <el-divider />
                <div>{{ article.content }}</div>
            </el-col>
        </el-row>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import articleApi from '@/api/article'

const route = useRoute()

const articleId = ref(0)
const article = ref({})

onMounted(() => {
    articleId.value = route.params.id
    articleApi.detail(articleId.value).then((res) => {
        article.value = res.data
    })
})
</script>

<style scoped>
.article-detail-page {
  max-width: 600px;
}
</style>
