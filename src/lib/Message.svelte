<script lang="ts">
import {marked} from 'marked';
import type {MessageData} from './types'
export let message:MessageData;
</script>
{#if message.sender === 'ai'}
<div class="chat chat-end">
    <div class="chat-bubble">
      {@html marked(message.text)}
    </div>
  </div>

{:else}
<div class="chat chat-start">
    <div class="chat-bubble">
      {message.text}
    </div>
  </div>

{/if}