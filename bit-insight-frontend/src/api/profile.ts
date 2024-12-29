import request from '../utils/request';

export namespace ProfileApi {
  export interface ProfileResponse {
    username: string;
    email?: string;
    phone?: string;
    created_at?: string;
    updated_at?: string;
  }
}

const profileApi = {
  profile: () => {
    return request.get<ProfileApi.ProfileResponse>('/api/v1/profile');
  },
};

export default profileApi;