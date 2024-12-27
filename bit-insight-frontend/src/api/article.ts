import request from '../utils/request';

interface ListRequest {
  page: number;
  page_size: number;
}

const articleApi = {
  list: (data: ListRequest) => {
    return request.get('/api/v1/articles', { params: data });
  },
  detail: (id: number) => {
    return request.get(`/api/v1/articles/${id}`);
  },
  remove: (id: number) => {
    return request.delete(`/api/v1/articles/${id}`);
  },
  create: (data: any) => {
    return request.post('/api/v1/articles', data);
  },
};

export default articleApi;

