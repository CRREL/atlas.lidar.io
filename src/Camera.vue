<template>
  <p v-if="errored">
    There was an error retriving {{ id }}.
  </p>

  <div v-else-if="camera">
    <h1 class="display-3">
      {{ camera.name }}
    </h1>

    <p class="lead">{{ camera.description }}</p>

    <p class="text-muted text-monospace">{{ camera.id }}</p>

    <div v-if="camera.latest_image">
      <b-img :src="camera.latest_image.url" fluid />
    </div>
  </div>

  <p v-else>
    Loading {{ id }}...
  </p>
</template>

<script>
import CameraMixin from './CameraMixin.vue'

export default {
  mixins: [CameraMixin]
}
</script>
