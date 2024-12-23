import request from '../utils/request';

const profileApi = {
  profile: () => {
    return request.get('/api/v1/profile');
  },
};

export default profileApi;
