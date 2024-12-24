<template>
    <div class="post-detail-page">
        <el-row>
            <el-col :span="24">
                <!-- {"id":100,"category_id":1,"title":"title50","author":"admin50","source":"","source_url":"","thumbnail":"","summary":"","content":"content50","created_at":"+002024-12-23T11:50:42.000000000Z","updated_at":"+002024-12-23T11:50:42.000000000Z"} -->
                <h1>{{ post.title }}</h1>
                <div>
                    <span><el-tag type="primary">{{ post.category_id }}</el-tag></span>
                    <el-divider direction="vertical" />
                    <span>{{ post.author }}</span>
                    <el-divider direction="vertical" />
                    <span><a href="{{ post.source_url }}">{{ post.source }}</a></span>
                    <el-divider direction="vertical" />
                    <span>{{ post.created_at }}</span>
                </div>
                <el-divider />
                <div>{{ post.content }}</div>
            </el-col>
        </el-row>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import postApi from '@/api/post'

const route = useRoute()

const postId = ref(0)
const post = ref({})

onMounted(() => {
    postId.value = route.params.id
    postApi.detail(postId.value).then((res) => {
        post.value = res.data
    })
})
</script>

<style scoped>
.post-detail-page {
  max-width: 600px;
}
</style>
