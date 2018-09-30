<template>
  <div id="site-list" class="mb-4">
    <b-card-group deck>
      <site-card id="south"></site-card>
      <site-card id="north"></site-card>
    </b-card-group>
  </div>
</template>

<script>
import SiteCard from './SiteCard.vue'

export default {
  components: {
    SiteCard
  }
}
</script>
