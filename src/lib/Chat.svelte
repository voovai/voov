<script lang="ts">
	import { onMount, afterUpdate, beforeUpdate } from 'svelte';
    import Message from './Message.svelte';	import type {MessageData} from './types'
	import {baseURL, sessionId, messages, url, content, message} from './store'
	import {insertMessage} from './services'
    import Button from './Button.svelte';
	import { marked } from 'marked';
	import MarkdownWrapper from './MarkdownWrapper.svelte';
    import { invoke } from "@tauri-apps/api/tauri"

    import { listen } from '@tauri-apps/api/event';
    import { ask } from '@tauri-apps/api/dialog';



// Remember to call unlisten() when you no longer need to listen for the event
// to avoid memory leaks and unnecessary processing
// e.g., unlisten().catch(console.error);



	let loading = false;
	let chatContainer: any;
	let contentDialog: any;

	export let messageText = '';
	let messageInput:any;

	onMount(async()=>{
		messageInput.focus();

        // Listen for the "event-name" event
        const unlisten = await listen('event-name', (event) => {
        console.log('Event received:', event);

        // Assuming the payload contains a message
        const message = event.payload.message;
        console.log('Message from Rust:', message);


    });

	});
	let autoscroll:any;

	beforeUpdate(() => {
		autoscroll = chatContainer && (chatContainer.offsetHeight + chatContainer.scrollTop) > (chatContainer.scrollHeight - 20);
		console.log("before",autoscroll)
	});

	afterUpdate(() => {
		if (autoscroll) window.scrollTo({ top: chatContainer.scrollHeight, behavior: 'smooth' });
		console.log("after",autoscroll)
	});

	function scrollToBottom() {
		window.scrollTo({ top: chatContainer.scrollHeight, behavior: 'smooth' })
  	}
	
	const newSession = ()=>{
      $sessionId = ""
      $content = ""
      $url = ""
      $message = ""
    }

	const viewContent = ()=>{
		contentDialog.showModal();
    }

	const sendMessage = async () => {
		const message = messageText
		messageText = ""
		if (message.trim() !== '') {
			$messages = [...$messages, { index: $messages.length+1, text: message, sender: 'user' }];
            var output:string = await invoke("chat", { message });
            $messages = [...$messages, { index: $messages.length+1, text: output , sender: 'ai' }];
			// loading = true;
			// await insertMessage($sessionId,'human',message);
			// // Add a response from the AI here or call an API for a dynamic response.
			// try {
			// 	const response = await fetch($baseURL + '/chat', {
			// 		method: 'POST',
			// 		headers: {
			// 			'Content-Type': 'application/json'
			// 		},
			// 		body: JSON.stringify({ session_id: $sessionId, message })
			// 	});
			// 	loading = false;

			// 	if (!response.ok) {
			// 		throw new Error('Error contacting the server.');
			// 	}

			// 	const data = await response.json();
			// 	$sessionId = data.session_id;
			// 	$messages = [...$messages, { index: $messages.length+1, text: data.ai_message, sender: 'ai' }];
			// 	await insertMessage($sessionId,'ai', data.ai_message);
			// } catch (error:any) {
			// 	alert('Error: ' + error.message);
			// }
		}
	};

	function handleKeydown(event:any) {
		if (event.key === 'Enter') {
			sendMessage();
		}
	}
</script>
<div>
	<dialog class=" w-6/12 z-50" bind:this={contentDialog}>
		<form method="dialog" class="w-11/12 max-w-5xl">
			<button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">✕</button>
			<h3 class="font-bold text-lg">Content</h3>
			<p class="py-4"><MarkdownWrapper>{@html marked($content)}</MarkdownWrapper></p>
		</form>
	</dialog>
	<div class="flex-1 flex-col border-t border-gray-300 bg-white p-4 rounded-lg" bind:this={chatContainer}>
		<div class="fixed right-4 top-24">
			
			<Button on:click={newSession}>
				New
			</Button>
			<Button on:click={viewContent}>
				View Content
			</Button>
		</div>
		<div id="messages-container" class="flex-grow">
			{#each $messages as message (message.index)}
				<Message {message}></Message>
			{/each}
			{#if loading}
				<div class="flex justify-center items-center">
					<div class="animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500" />
				</div>
			{/if}
		</div>
	</div>
</div>

	
<div class="fixed inset-x-0 bottom-0 bg-white py-2 px-4 border-t border-gray-300">
	<div class="flex p-4">
		<input
			type="text"
			bind:value={messageText}
			bind:this={messageInput}
			placeholder="Type your question..."
			on:keydown={handleKeydown}
			class="flex-1 p-2 border border-gray-300 rounded-l-md focus:outline-none focus:ring focus:border-blue-300"
		/>
		<button
			on:click={sendMessage}
			class="px-4 py-2 bg-blue-500 text-white font-semibold rounded-r-md focus:outline-none focus:ring focus:ring-blue-300"
			>Send</button
		>
	</div>
</div>

<style>
</style>
