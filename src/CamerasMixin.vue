<script>
import axios from 'axios'

export default {
  data () {
    return {
      cameras: null,
      errored: false
    }
  },
  mounted () {
    axios
      .get('http://localhost:3000/cameras')
      .then(response => (this.cameras = response.data))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
  }
}
</script>
