<script>
import axios from 'axios'

var getCamera = function (id) {
  axios
    .get('http://localhost:3000/cameras/' + this.id)
    .then(response => (this.camera = response.data))
    .catch(error => {
      console.log(error)
      this.errored = true
    })
}

export default {
  props: ['id'],
  data () {
    return {
      camera: null,
      errored: false
    }
  },
  mounted: getCamera,
  watch: {
    id: getCamera
  }
}
</script>
