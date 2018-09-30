<script>
import axios from 'axios'

var getSite = function (id) {
  axios
    .get('http://localhost:3000/sites/' + this.id)
    .then(response => (this.site = response.data))
    .catch(error => {
      console.log(error)
      this.errored = true
    })
}

export default {
  props: ['id'],
  data () {
    return {
      errored: false,
      site: null
    }
  },
  mounted: getSite,
  watch: {
    id: getSite
  }
}
</script>
