<template>
  <b-card
    class="my-3"
    v-if="camera"
    :img-src="camera.latest_image ? camera.latest_image.url : ''"
    img-top
    :title="camera.name"
    >

    <h6 class="card-subtitle mb-2 text-muted text-monospace">{{ camera.id }}</h6>

    <p class="card-text">
      {{ camera.description }}
    </p>

    <router-link :to="{ name: 'camera', params: { id: camera.id } }" tag="button" class="btn btn-outline-dark">
      See more images
    </router-link>

    <span slot="footer" v-if="camera.latest_image">
      Latest image taken {{ camera.latest_image.datetime | moment("from") }}
    </span>
  </b-card>

  <b-card v-else class="my-3">
    Loading {{ id }}...
  </b-card>
</template>

<script>
export default {
  props: ['id'],
  computed: {
    camera () {
      return this.$store.getters.camera(this.id)
    }
  }
}
</script>
