<template>
  <p v-if="errored">
    There was an error retriving {{ id }}.
  </p>

  <div v-else-if="site">
    <h1 class="display-3">
      {{ site.name }}
    </h1>

    <p class="text-muted text-monospace">{{ site.id }}</p>

    <div v-if="site.latest_heartbeat">
      <h4>Heartbeat</h4>
      <pre class="text-left"><code>{{ site.latest_heartbeat }}</code></pre>
    </div>
  </div>

  <p v-else>
    Loading {{ id }}...
  </p>
</template>

<script>
import SiteMixin from './SiteMixin.vue'

export default {
  mixins: [SiteMixin]
}
</script>
