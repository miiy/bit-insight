import request from '../utils/request';

const authApi = {
  login: (data) => {
    return request.post('/api/v1/auth/login', data);
  },
  register: (data) => {
    return request.post('/api/v1/auth/register', data);
  },
  logout: () => {
    return request.post('/api/v1/auth/logout');
  },
};

export default authApi;