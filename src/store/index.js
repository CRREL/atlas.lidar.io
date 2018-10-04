import Vue from 'vue'
import Vuex from 'vuex'
import cameras from './modules/cameras'

Vue.use(Vuex)

export default new Vuex.Store({
  modules: {
    cameras
  },
  strict: process.env.NODE_ENV !== 'production'
})
