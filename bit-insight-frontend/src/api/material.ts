import request from '../utils/request';

interface ListRequest {
  page: number;
  page_size: number;
}

const materialApi = {
  list: (data: ListRequest) => {
    return request.get('/api/v1/materials', { params: data });
  },
  detail: (id: number) => {
    return request.get(`/api/v1/materials/${id}`);
  },
  remove: (id: number) => {
    return request.delete(`/api/v1/materials/${id}`);
  },
  create: (data: any) => {
    return request.post('/api/v1/materials', data);
  },
};

export default materialApi;
