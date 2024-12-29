<template>
    <div class="material-detail-page">
        <el-row>
            <el-col :span="24">
                <h1>{{ material?.title }}</h1>
                <div>
                    <span><el-tag type="primary">{{ material?.category_id }}</el-tag></span>
                    <el-divider direction="vertical" />
                    <span>{{ material?.author }}</span>
                    <el-divider direction="vertical" />
                    <span><a href="{{ material?.source_url }}" target="_blank">{{ material?.source }}</a></span>
                    <el-divider direction="vertical" />
                    <span>{{ material?.created_at }}</span>
                </div>
                <el-divider />
                <div v-html="material?.content"></div>
            </el-col>
        </el-row>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import materialApi from '@/api/material'
import type { MaterialApi } from '@/api'
import { useRoute } from 'vue-router'

const route = useRoute()

const materialId = ref(Number(route.params.id))
const material = ref<MaterialApi.DetailResponse>()

const getMaterialDetail = async () => {
    const resp = await materialApi.detail(materialId.value)
    material.value = resp.data
}

getMaterialDetail()
</script>

<style scoped>
.material-detail-page {
  max-width: 600px;
}
</style>
