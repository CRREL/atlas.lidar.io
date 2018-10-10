import axios from 'axios'

const resource = function (path) {
  return process.env.VUE_APP_API_URL + '/' + path
}

const cameras = function (callback) {
  return axios.get(resource('cameras'))
}

const cameraImages = function (id, callback) {
  return axios.get(resource('cameras/' + id + '/images'))
}

export default {
  cameras,
  cameraImages
}
