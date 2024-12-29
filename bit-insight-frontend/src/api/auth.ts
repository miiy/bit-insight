import request from '../utils/request';

export namespace AuthApi {
  export interface LoginRequest {
    username: string;
    password: string;
  }

  export interface LoginResponse {
    token_type: string;
    access_token: string;
    expires_in: number;
  }

  export interface RegisterRequest {
    username: string;
    password: string;
    password_confirmed: string;
    email: string;
  }

  export interface RegisterResponse {
    message: string;
  }
}

const authApi = {
  login: (data: AuthApi.LoginRequest) => {
    return request.post<AuthApi.LoginResponse>('/api/v1/auth/login', data);
  },
  register: (data: AuthApi.RegisterRequest) => {
    return request.post<AuthApi.RegisterResponse>('/api/v1/auth/register', data);
  },
  logout: () => {
    return request.post('/api/v1/auth/logout');
  },
};

export default authApi;