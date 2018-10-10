import axios from 'axios'

const resource = function (path) {
  return process.env.VUE_APP_API_URL + '/' + path
}

const cameras = function (callback) {
  axios
    .get(resource('cameras'))
    .then(response => callback(response.data))
}

const cameraImages = function (id, callback) {
  axios
    .get(resource('camera/' + id + '/images'))
    .then(response => callback(response.data))
}

export default {
  cameras,
  cameraImages
}
