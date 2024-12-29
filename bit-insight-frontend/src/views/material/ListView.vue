<template>
  <el-table :data="tableData" style="width: 100%">
    <el-table-column prop="category_id" label="Category" width="120" />
    <el-table-column prop="title" label="Title" width="120" />
    <el-table-column prop="author" label="Author" width="120" />
    <el-table-column prop="source" label="Source" width="120" />
    <el-table-column prop="source_url" label="Source URL" width="120" />
    <el-table-column prop="thumbnail" label="Thumbnail" width="120" />
    <el-table-column prop="summary" label="Summary" width="120" />
    <el-table-column prop="created_at" label="Created At" width="120" />
    <el-table-column fixed="right" label="Operations" min-width="120">
      <template #default="scope">
        <el-button link type="primary" size="small" @click="handleDetail(scope.row)">
          Detail
        </el-button>
      </template>
    </el-table-column>
  </el-table>
  <div class="pagination">
    <el-pagination background layout="prev, pager, next"
    :current-page="page"
    :page-size="per_page"
    :total="total"
    @current-change="handlePageChange"
    @size-change="handleSizeChange" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import materialApi from '@/api/material'
import type { MaterialApi } from '@/api/material'

const router = useRouter()

const page = ref(1)
const per_page = ref(10)
const total_pages = ref(1)
const total = ref(0)
const tableData = ref<MaterialApi.ListResponseItem[]>([])

const handlePageChange = (pg: number) => {
  page.value = pg
  getTableData()
}

const handleSizeChange = (size: number) => {
  per_page.value = size
  getTableData()
}

const getTableData = async () => {
  const req: MaterialApi.ListRequest = {
    page: page.value,
    per_page: per_page.value,
  }
  const resp = await materialApi.list(req)
  const res = resp.data

  page.value = res.page
  per_page.value = res.per_page
  total_pages.value = res.total_pages
  total.value = res.total_pages * res.per_page
  tableData.value = res.data
}

getTableData()

const handleDetail = (row: any) => {
  router.push({ name: 'MaterialDetail', params: { id: row.id } })
}

</script>

<style scoped>
.pagination {
  margin: 20px 0;
  display: flex;
  justify-content: center;
}

.el-pagination {
  margin: 0;
}
</style>
