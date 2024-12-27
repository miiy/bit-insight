import request from '../utils/request';

const userApi = {
  info: (id: number) => {
    return request.get(`/api/v1/users/${id}`);
  },
};

export default userApi;
