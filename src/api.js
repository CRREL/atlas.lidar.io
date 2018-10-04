import axios from 'axios'

const resource = function (path) {
  return process.env.VUE_APP_API_URL + '/' + path
}

const cameras = function (callback) {
  axios
    .get(resource('cameras'))
    .then(response => callback(response.data))
}

export default {
  cameras
}
