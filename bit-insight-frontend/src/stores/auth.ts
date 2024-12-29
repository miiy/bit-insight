import { defineStore } from 'pinia';
import authApi from '../api/auth';
import profileApi from '../api/profile';

export const useAuthStore = defineStore('authStore', {
  // state properties
  state: () => ({
    token: null,
    user: null,
  }),
  // computed properties
  getters: {
    isAuthenticated: (state): boolean => state.token !== null,
  },
  // actions
  actions: {
    async login(data: {username: string, password: string}): Promise<any> {
      return new Promise((resolve, reject) => {
        authApi.login(data).then((res) => {
          // update pinia state
          this.token = res.data;
          profileApi.profile().then((res) => {
            this.user = {username: res.data.username};
            resolve(res);
          }).catch((err) => {
            this.token = null;
            reject(err);
          });
        }).catch((err) => {
          reject(err);
        });
      });
    },
    async logout(): Promise<any> {
      return new Promise((resolve, reject) => {
        authApi.logout().then((res) => {
          // update pinia state
          this.token = null;
          this.user = null;
          resolve(res);
        }).catch((err) => {
          reject(err);
        });
      });
    },
  },
  persist: true,
});
