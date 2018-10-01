<template>
  <div class="dashboard">
    <h1 class="display-3">ATLAS</h1>

    <p class="mb-4 lead">
      Remote glacier monitoring.
    </p>

    <h2 class="display-5 mt-5">Sites</h2>

    <site-list></site-list>

    <h2 class="display-5 mt-5">Cameras</h2>

    <camera-list></camera-list>
  </div>
</template>

<script>
import CameraList from './CameraList.vue'
import SiteList from './SiteList.vue'

export default {
  components: {
    CameraList,
    SiteList
  }
}
</script>
