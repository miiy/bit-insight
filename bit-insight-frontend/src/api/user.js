import request from '../utils/request';

const userApi = {
  profile: () => {
    return request.get('/api/v1/users/profile');
  },
};

export default userApi;
