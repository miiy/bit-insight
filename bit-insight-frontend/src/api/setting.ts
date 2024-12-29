import request from '../utils/request';

export namespace SettingApi {

  export interface GetRequest {
    key: string;
  }

  export interface GetResponse {
    value: any;
  }
  
  export interface UpdateRequest {
    key: string;
    value: any;
  }

  export interface UpdateResponse {
  }
}

const settingApi = {
  get: (data: SettingApi.GetRequest) => {
    return request.get<SettingApi.GetResponse>(`/api/v1/settings/${data.key}`);
  },
  update: (data: SettingApi.UpdateRequest) => {
    return request.put<SettingApi.UpdateResponse>(`/api/v1/settings/${data.key}`, data.value);
  },
};

export default settingApi;
