import axios from 'axios';
import { useAuthStore } from '../stores';

let authStore = useAuthStore();

const config = {
    baseURL: process.env.baseURL || process.env.apiUrl || "",
    timeout: 1000,
};

const request = axios.create(config);

// interceptors
// https://github.com/axios/axios?tab=readme-ov-file#interceptors

// Add a request interceptor
request.interceptors.request.use(
  // Do something before request is sent
  function (config) {
    const token = authStore.token;
    if (token) {
      config.headers.Authorization = token.token_type + ' ' + token.access_token;
    }
    // const user = store.getters.user
    // if (token.access_token && !user.name) {
    //   console.log('get user')
    //   store.dispatch('getUser').then(resp => {
    //     console.log('set user' + resp.data.name)
    //   }).catch(err => {
    //     console.log(err)
    //     router.push({ name: 'Login' })
    //   })
    // }
    return config;
  },
  function (error) {
    // Do something with request error
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
    return Promise.reject(error);
  }
);

export default request;
