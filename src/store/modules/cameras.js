import api from '../../api'

const state = {
  cameras: []
}

const getters = {
  camera: (state) => (id) => {
    return state.cameras.find(camera => camera.id === id)
  }
}

const actions = {
  fetchCameras ({ commit }) {
    api.cameras(cameras => {
      commit('setCameras', cameras)
    })
  }
}

const mutations = {
  setCameras (state, cameras) {
    state.cameras = cameras
  }
}

export default {
  actions,
  getters,
  mutations,
  state
}