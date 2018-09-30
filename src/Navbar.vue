<template>
  <b-navbar
    toggleable
    type="light"
    class="container"
    >
    <b-navbar-toggle target="nav_collapse"></b-navbar-toggle>
    <b-navbar-brand href="/">ATLAS</b-navbar-brand>
    <b-collapse is-nav id="nav_collapse">
      <b-navbar-nav>
        <router-link :to="{ name: 'dashboard' }" class="nav-link">Dashboard</router-link>

        <!-- TODO this is so refactorable -->
        <b-nav-item-dropdown text="Sites">
          <div v-if="sites">
            <router-link
               v-for="site in sites"
               :key="site.id"
               :to="{ name: 'site', params: { id: site.id } }"
               class="dropdown-item"
               >
              {{ site.name }}
            </router-link>
          </div>

          <div v-else-if="errored">
            <b-dropdown-item>Error retriving site list.</b-dropdown-item>
          </div>

          <div v-else>
            <b-dropdown-item>Loading...</b-dropdown-item>
          </div>
        </b-nav-item-dropdown>

        <b-nav-item-dropdown text="Cameras">
          <div v-if="cameras">
            <router-link
               v-for="camera in cameras"
               :key="camera.id"
               :to="{ name: 'camera', params: { id: camera.id } }"
               class="dropdown-item"
               >
              {{ camera.name }}
            </router-link>
          </div>

          <div v-else-if="errored">
            <b-dropdown-item>Error retriving camera list.</b-dropdown-item>
          </div>

          <div v-else>
            <b-dropdown-item>Loading...</b-dropdown-item>
          </div>
        </b-nav-item-dropdown>

      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
</template>

<script>
import CamerasMixin from './CamerasMixin.vue'
import SitesMixin from './SitesMixin.vue'

export default {
  mixins: [CamerasMixin, SitesMixin]
}
</script>
