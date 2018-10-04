import axios from 'axios'

const resource = function (path) {
  let baseUrl
  if (process.env.NODE_ENV === 'production') {
    baseUrl = 'http://api.glac.io'
  } else {
    baseUrl = 'http://api-dev.glac.io'
  }
  return baseUrl + '/' + path
}

const cameras = function (callback) {
  axios
    .get(resource('cameras'))
    .then(response => callback(response.data))
}

export default {
  cameras
}
