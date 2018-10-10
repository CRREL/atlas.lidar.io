<template>
  <div class="camera" v-if="camera">
    <h1 class="display-4">{{ camera.name }}</h1>

    <p class="lead">
    {{ camera.description }}
    </p>

    <div v-if="images">
      {{ images[0] }}
    </div>
  </div>
</template>

<script>
import api from '../api'

export default {
  data () {
    return {
      images: null
    }
  },
  props: ['id'],
  computed: {
    camera () {
      return this.$store.getters.camera(this.id)
    }
  },
  methods: {
    fetchImages: function () {
      api.cameraImages(this.id, 0).then(response => { this.images = response.data })
    }
  },
  mounted () {
    this.fetchImages()
  },
  watch: {
    id: function (_newId, _oldId) {
      this.images = null
      this.fetchImages()
    }
  }
}
</script>
