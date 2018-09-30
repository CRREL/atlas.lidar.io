<script>
import axios from 'axios'

export default {
  data () {
    return {
      sites: null,
      errored: false
    }
  },
  mounted () {
    axios
      .get('http://localhost:3000/sites')
      .then(response => (this.sites = response.data))
      .catch(error => {
        console.log(error)
        this.errored = true
      })
  }
}
</script>
