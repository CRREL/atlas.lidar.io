import Vue from 'vue'
import VueRouter from 'vue-router'
import VueMoment from 'vue-moment'
import { library } from '@fortawesome/fontawesome-svg-core'
import { faCoffee } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import BootstrapVue from 'bootstrap-vue'
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

import App from './App.vue'
import Dashboard from './Dashboard.vue'
import Camera from './Camera.vue'
import Site from './Site.vue'

library.add(faCoffee)

Vue.config.productionTip = false
Vue.use(VueRouter)
Vue.use(VueMoment)
Vue.use(BootstrapVue)
Vue.component('font-awesome-icon', FontAwesomeIcon)

const routes = [
  { path: '/', redirect: { name: 'dashboard' } },
  { path: '/dashboard', component: Dashboard, name: 'dashboard' },
  { path: '/cameras/:id', component: Camera, name: 'camera', props: true },
  { path: '/sites/:id', component: Site, name: 'site', props: true }
]

const router = new VueRouter({
  mode: 'history',
  linkActiveClass: 'active',
  routes
})

new Vue({
  render: h => h(App),
  router
}).$mount('#app')
