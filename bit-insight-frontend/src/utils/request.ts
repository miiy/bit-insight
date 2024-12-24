import axios from 'axios';
import { useAuthStore } from '../stores';

const config = {
    baseURL: import.meta.env.VITE_BASE_URL || import.meta.env.VITE_API_URL || "",
    timeout: 1000,
};

const request = axios.create(config);

// interceptors
// https://github.com/axios/axios?tab=readme-ov-file#interceptors

// Add a request interceptor
request.interceptors.request.use(
  // Do something before request is sent
  function (config) {
    let authStore = useAuthStore();
    const token = authStore.token;
    if (token) {
      config.headers.Authorization = token.token_type + ' ' + token.access_token;
    }
    return config;
  },
  function (error) {
    // Do something with request error
    console.log(error)
    return Promise.reject(error);
  }
);

// Add a response interceptor
request.interceptors.response.use(
  function (response) {
    // Any status code that lie within the range of 2xx cause this function to trigger
    // Do something with response data
    return response;
  },
  function (error) {
    // Any status codes that falls outside the range of 2xx cause this function to trigger
    // Do something with response error
    console.log(error)
    return Promise.reject(error.response);
  }
);

export default request;
