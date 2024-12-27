import request from '../utils/request';

const settingApi = {
  get: (key: string) => {
    return request.get(`/api/v1/settings/${key}`);
  },
  update: (key: string, value: any) => {
    return request.put(`/api/v1/settings/${key}`, value);
  },
};

export default settingApi;
