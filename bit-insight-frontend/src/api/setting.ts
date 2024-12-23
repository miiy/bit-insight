import request from '../utils/request';

const settingApi = {
  get: (key) => {
    return request.get(`/api/v1/settings/${key}`);
  },
  update: (key, value) => {
    return request.put(`/api/v1/settings/${key}`, value);
  },
};

export default settingApi;
