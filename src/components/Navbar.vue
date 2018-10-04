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

        <b-nav-item-dropdown text="Cameras">
          <router-link
            v-for="camera in cameras"
            :key="camera.id"
            :to="{ name: 'camera', params: { id: camera.id } }"
            class="dropdown-item"
            >
            {{ camera.name }}
          </router-link>
        </b-nav-item-dropdown>

      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
</template>

<script>
export default {
  computed: {
    cameras () {
      return this.$store.state.cameras.cameras
    }
  }
}
</script>
