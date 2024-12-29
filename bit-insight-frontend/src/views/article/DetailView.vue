<template>
    <div class="article-detail-page">
        <el-row>
            <el-col :span="24">
                <h1>{{ article?.title }}</h1>
                <div>
                    <span><el-tag type="primary">{{ article?.category_id }}</el-tag></span>
                    <el-divider direction="vertical" />
                    <span>{{ article?.author }}</span>
                    <el-divider direction="vertical" />
                    <span><a href="{{ article?.source_url }}" target="_blank">{{ article?.source }}</a></span>
                    <el-divider direction="vertical" />
                    <span>{{ article?.created_at }}</span>
                </div>
                <el-divider />
                <div>{{ article?.content }}</div>
            </el-col>
        </el-row>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import articleApi from '@/api/article'
import type { ArticleApi } from '@/api'

const route = useRoute()

const articleId = ref(Number(route.params.id))
const article = ref<ArticleApi.DetailResponse>()

const getArticleDetail = async () => {
    const resp = await articleApi.detail(articleId.value)
    article.value = resp.data
}

getArticleDetail()
</script>

<style scoped>
.article-detail-page {
  max-width: 600px;
}
</style>
