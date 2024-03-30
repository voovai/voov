<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let dirName = "/Users/gavi/Downloads";
  let fileList = ""

  async function list_files(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    fileList = await invoke("list_files", { dirName })
    console.log(fileList)
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={list_files}>
    <input id="greet-input" placeholder="Enter a folder name..." bind:value={dirName} />
    <button type="submit">Get List Files</button>
  </form>
  {dirName}
  <p>{fileList}</p>

</div>