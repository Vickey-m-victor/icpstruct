<template>
  <div id="app">
    <button @click="login">Login with Internet Identity</button>
    <p>Your Principal: {{ principal }}</p>
  </div>
</template>

<script>
import { AuthClient } from '@dfinity/auth-client';
import { Actor, HttpAgent } from '@dfinity/agent';

export default {
  data() {
    return {
      principal: '',
    };
  },
  methods: {
    async login() {
      const authClient = await AuthClient.create();
      const identity = await authClient.login({
        identityProvider: 'http://localhost:4943/?canisterId=internet_identity',
      });

      const agent = new HttpAgent({ identity });

      const { idlFactory: myProjectIdl, canisterId: myProjectId } = await this.importModules();

      const myProjectActor = Actor.createActor(myProjectIdl, {
        agent,
        canisterId: myProjectId,
      });

      this.principal = await myProjectActor.whoami();
    },
    async importModules() {
      return await import('dfx-generated/my_project_backend');
    },
  },
};
</script>