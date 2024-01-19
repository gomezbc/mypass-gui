<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  import LoginEntry from "@/components/LoginEntry.svelte";

  type Login = {
    domain: string;
    credentials: Credential[];
  };
  type Credential = {
    email: string;
    usr: string;
    pass: string;
  };

  async function getLogins(): Promise<Login[]> {
    let logins = (await invoke("get_logins")) as Login[];
    console.log(logins);
    return logins;
  }

  let logins: Login[] = [];

  getLogins().then((result) => {
    logins = result;
    logins.forEach((login) => {
      login.credentials.forEach((credential) => {
        credential.pass = "•".repeat(8);
      });
    });
  });
</script>

<ul>
  {#each logins as login}
    <LoginEntry domain={login.domain} credentials={login.credentials} />
  {/each}
</ul>
