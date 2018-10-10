<template>
  <div class="camera" v-if="camera">
    <h1 class="display-4">{{ camera.name }}</h1>

    <p class="lead">
    {{ camera.description }}
    </p>

    <b-row>
      <b-col offset="3" cols="6">
        <b-img :src="camera.latest_image.url" fluid />
      </b-col>
    </b-row>

    <image-picker v-if="images" :tree="images">
    </image-picker>
  </div>
</template>

<script>
import api from '../api'
import ImagePicker from './ImagePicker.vue'

export default {
  components: {
    ImagePicker
  },
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
      api.cameraImages(this.id, 0).then(response => { this.images = this.buildTree(response.data) })
    },
    buildTree: function (images) {
      let tree = {}
      for (let i = 0; i < images.length; i++) {
        const image = images[i]
        const date = new Date(image.datetime)
        const year = date.getFullYear()
        const month = date.getMonth()
        const day = date.getDay()
        if (!tree[year]) {
          tree[year] = {}
        }
        if (!tree[year][month]) {
          tree[year][month] = {}
        }
        if (!tree[year][month][day]) {
          tree[year][month][day] = []
        }
        tree[year][month][day].push(image)
      }
      return tree
    }
  },
  mounted () {
    this.fetchImages()
  },
  watch: {
    id: function () {
      this.images = null
      this.fetchImages()
    }
  }
}
</script>
