<script lang="ts">
	import { Dialog, DialogOverlay, Transition, TransitionChild } from '@rgossiaux/svelte-headlessui';
	import type { Gradient } from '$types/gradient';
	import GradientBackground from './GradientBackground.svelte';

	export let open: boolean;
	export let titleGradient: Gradient;
</script>

<!-- https://github.com/rgossiaux/svelte-headlessui/blob/master/src/routes/dialog/index.svelte -->
<Transition show={open}>
	<Dialog on:close>
		<div class="fixed inset-0 z-[100] overflow-y-auto">
			<div class="min-h-screen text-center block p-0">
				<TransitionChild
					enter="ease-out duration-300"
					enterFrom="opacity-0"
					enterTo="opacity-75"
					leave="ease-in duration-200"
					leaveFrom="opacity-75"
					leaveTo="opacity-0"
					entered="opacity-75"
				>
					<DialogOverlay class="fixed inset-0 bg-black transition-opacity" />
				</TransitionChild>

				<TransitionChild
					enter="ease-out transform duration-300"
					enterFrom="opacity-0 scale-95"
					enterTo="opacity-100 scale-100"
					leave="ease-in transform duration-200"
					leaveFrom="opacity-100 scale-100"
					leaveTo="opacity-0 scale-95"
				>
					<!-- This element is to trick the browser into centering the modal contents. -->
					<span class="inline-block align-middle h-screen" aria-hidden="true"> &#8203; </span>
					<div
						class="inline-block content-center bg-white rounded-3xl text-left shadow-xl transform transition-all my-8 align-middle max-w-[800px] max-h-[600px] w-full"
					>
						<div class="flex flex-col items-center relative">
							<!-- mb-8 determined by half of height of child (h-16)
                                due to moving up by half. -->
							<div class="relative mb-8 w-full max-w-[480px]">
								<GradientBackground
									gradient={titleGradient}
									class="absolute inset-0 -translate-y-1/2 rounded-full flex flex-row items-center justify-center gap-8 h-16 px-8 py-2"
								>
									<div class="w-12 h-12"><slot name="icon" /></div>
									<div class="text-3xl font-bold"><slot name="title" /></div>
								</GradientBackground>
							</div>
							<slot name="content" />
							<div class="translate-y-1/2"><slot name="close" /></div>
						</div>
					</div>
				</TransitionChild>
			</div>
		</div>
	</Dialog>
</Transition>
