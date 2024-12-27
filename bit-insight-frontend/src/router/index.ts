import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/auth/LoginView.vue'),
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/auth/RegisterView.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/setting/SettingView.vue'),
    },
    {
      path: '/profile',
      name: 'profile',
      component: () => import('../views/profile/ProfileView.vue'),
    },
    // Materials
    {
      path: '/materials',
      name: 'materials',
      component: () => import('../views/material/ListView.vue'),
    },
    {
      path: '/materials/:id',
      name: 'MaterialDetail',
      component: () => import('../views/material/DetailView.vue'),
    },
    // Articles
    {
      path: '/articles',
      name: 'articles',
      component: () => import('../views/article/ListView.vue'),
    },
    {
      path: '/articles/:id',
      name: 'ArticleDetail',
      component: () => import('../views/article/DetailView.vue'),
    },
    {
      path: '/articles/create',
      name: 'ArticleCreate',
      component: () => import('../views/article/CreateView.vue'),
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
  ],
})

export default router
