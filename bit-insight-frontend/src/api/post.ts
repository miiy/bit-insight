import request from '../utils/request';

const postApi = {
  list: (params) => {
    return request.get('/api/v1/posts', { params });
  },
  detail: (id) => {
    return request.get(`/api/v1/posts/${id}`);
  },
  remove: (id) => {
    return request.delete(`/api/v1/posts/${id}`);
  },
  create: (data) => {
    return request.post('/api/v1/posts', data);
  },
};

export default postApi;

