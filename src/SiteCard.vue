<template>
  <b-card v-if="errored">
    Error when loading {{ id }}.
  </b-card>

  <b-card v-else-if="site"
    :title="site.name"
    >
    <h6 class="card-subtitle mb-2 text-muted text-monospace">{{ site.id }}</h6>

    <p v-if="site.latest_heartbeat">
      The latest heartbeat was received {{ site.latest_heartbeat.datetime | moment("from") }}.
    </p>

    <router-link :to="{ name: 'site', params: { id: site.id } }" tag="button" class="btn btn-outline-dark">
      See more information
    </router-link>
  </b-card>

  <b-card v-else>
    Loading {{ id }}...
  </b-card>
</template>

<script>
import SiteMixin from './SiteMixin.vue'

export default {
  mixins: [SiteMixin]
}
</script>
