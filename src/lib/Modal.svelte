<script>
// @ts-nocheck

	import { fade, fly } from "svelte/transition";
	//import { quintOut } from "svelte/easing";
  
	export let open = false;
	export let showBackdrop = true;
	export let onClosed;
	export let title = 'Modal title';
  
	const modalClose = (data) => {
	  open = false;
	  if (onClosed) {
		onClosed(data);
	  }
	}
  
  </script>
  
  {#if open}
	<div class="modal" id="sampleModal" tabindex="-1" role="dialog" 
	aria-labelledby="sampleModalLabel" aria-hidden={false}>
	  <div class="modal-dialog" role="document" >
		<div class="modal-content">
		  <div class="modal-header">
			<h5 class="modal-title">{title}</h5>
			<button type="button" class="close" data-dismiss="modal"
			  on:click={() => modalClose()}>
			  <i class="bi-x"/>
			</button>
		  </div>
		  <div class="modal-body">
			<slot></slot>
		  </div>
		  <div class="modal-footer">
			<button type="button" class="btn btn-light button" data-dismiss="modal" 
			on:click={() => modalClose(false)}>Cancel</button>
			<button type="button" class="btn btn-primary button" 
			on:click={() => modalClose(true)}>Ok</button>
		  </div>
		</div>
	  </div>
	</div>
	{#if showBackdrop}
	<div class="modal-backdrop show" transition:fade={{ duration: 50 }} />
	{/if}
  {/if}
  
  <style>
	.modal {
	  display: block;
	}
	.close {
		text-align: center;
		width: 24px;
		padding: 0;
		margin: 0;
	}
	.button {
		width: 100px;
	}
  </style>