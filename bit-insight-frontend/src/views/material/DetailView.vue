<template>
    <div class="material-detail-page">
        <el-row>
            <el-col :span="24">
                <!-- {"id":100,"category_id":1,"title":"title50","author":"admin50","source":"","source_url":"","thumbnail":"","summary":"","content":"content50","created_at":"+002024-12-23T11:50:42.000000000Z","updated_at":"+002024-12-23T11:50:42.000000000Z"} -->
                <h1>{{ material.title }}</h1>
                <div>
                    <span><el-tag type="primary">{{ material.category_id }}</el-tag></span>
                    <el-divider direction="vertical" />
                    <span>{{ material.author }}</span>
                    <el-divider direction="vertical" />
                    <span><a href="{{ material.source_url }}">{{ material.source }}</a></span>
                    <el-divider direction="vertical" />
                    <span>{{ material.created_at }}</span>
                </div>
                <el-divider />
                <div>{{ material.content }}</div>
            </el-col>
        </el-row>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import materialApi from '@/api/material'

const route = useRoute()

const materialId = ref(0)
const material = ref({})

onMounted(() => {
    materialId.value = route.params.id
    materialApi.detail(materialId.value).then((res) => {
        material.value = res.data
    })
})
</script>

<style scoped>
.material-detail-page {
  max-width: 600px;
}
</style>
