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
      await authClient.login({
        identityProvider: `http://localhost:4943/?canisterId=${import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY}`,
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          const agent = new HttpAgent({ identity });

          const { idlFactory: myProjectIdl } = await this.importModules();

          const myProjectId = import.meta.env.VITE_CANISTER_ID_MY_PROJECT_BACKEND;

          const myProjectActor = Actor.createActor(myProjectIdl, {
            agent,
            canisterId: myProjectId,
          });

          const principal = await myProjectActor.whoami();
          console.log('Principal:', principal); // Log the principal value to the console
          this.principal = principal.toText();
        },
      });
    },
    async importModules() {
      return await import('dfx-generated/my_project_backend');
    },
  },
};
</script>