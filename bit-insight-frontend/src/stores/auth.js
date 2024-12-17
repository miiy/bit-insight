import { defineStore } from 'pinia';
import authApi from '../api/auth';

export const useAuthStore = defineStore('auth', {
  // state properties
  state: () => ({
    token: null,
    user: null,
  }),
  // computed properties
  getters: {
    isAuthenticated: (state) => state.token !== null,
  },
  // actions
  actions: {
    async login(data) {
      return new Promise((resolve, reject) => {
        authApi.login(data).then((res) => {
          // update pinia state
          this.token = res.data;

          resolve(res);
        }).catch((err) => {
          reject(err);
        });
      });
    },
    async logout() {
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
