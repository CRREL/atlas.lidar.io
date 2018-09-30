<template>
  <b-card v-if="errored" class="my-3">
    <p class="card-text">
      There was an error loading {{ id }}.
    </p>
  </b-card>

  <b-card
    class="my-3"
    v-else-if="camera"
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
    <p class="card-text font-weight-light font-italic text-muted">
      Loading {{ id }}.
    </p>
  </b-card>
</template>

<script>
import CameraMixin from './CameraMixin.vue'

export default {
  mixins: [CameraMixin]
}
</script>
