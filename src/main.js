import Vue from 'vue'
import VueRouter from 'vue-router'
import VueMoment from 'vue-moment'
import Vuex from 'vuex'
import { library } from '@fortawesome/fontawesome-svg-core'
import { faCoffee } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import BootstrapVue from 'bootstrap-vue'

import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

import App from './components/App.vue'
import Dashboard from './components/Dashboard.vue'
import Camera from './components/Camera.vue'
import store from './store'

library.add(faCoffee)

Vue.component('font-awesome-icon', FontAwesomeIcon)
Vue.config.productionTip = false
Vue.use(BootstrapVue)
Vue.use(VueMoment)
Vue.use(VueRouter)
Vue.use(Vuex)

const routes = [
  { path: '/', redirect: { name: 'dashboard' } },
  { path: '/dashboard', component: Dashboard, name: 'dashboard' },
  { path: '/camera/:id', component: Camera, name: 'camera', props: true }
]

const router = new VueRouter({
  mode: 'history',
  linkActiveClass: 'active',
  routes
})

new Vue({
  render: h => h(App),
  router,
  store
}).$mount('#app')
