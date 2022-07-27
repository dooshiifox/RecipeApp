<script context="module" lang="ts">
	import PageError from '$lib/PageError.svelte';
	import RecipeHeader from '$lib/recipe-view/RecipeHeader.svelte';
	import Ingredients from '$lib/recipe-view/page/ingredients/Ingredients.svelte';
	import Method from '$lib/recipe-view/page/method/Method.svelte';

	import { Recipe } from '$types/Recipe';
	import type { APIErrorResponse, APIResponse } from '$utils/fetch';

	export const prerender = true;

	interface Load<T> {
		params: T;
		fetch: typeof fetch;
	}

	export async function load({ params, fetch }: Load<{ short: string }>) {
		const recipe = await Recipe.getByShort(params.short, fetch).catch(
			(e: APIErrorResponse<APIErrorResponse['error']>['error']) => {
				return e;
			}
		);

		if (recipe === undefined)
			// Returns an APIErrorResponse
			return {
				props: {
					response: {
						success: false,
						message: "If you're seeing this, something went quite wrong!"
					}
				}
			};

		if ('message' in recipe) {
			// Returns an APIErrorResponse
			return {
				props: {
					response: {
						success: false,
						error: recipe
					}
				}
			};
		}

		// Returns an APISuccessResponse<Recipe>
		return {
			props: {
				response: {
					success: true,
					data: recipe
				}
			}
		};
	}
</script>

<script lang="ts">
	export let response: APIResponse<Recipe, APIErrorResponse['error']>;
	/** NOTE: Only use if you know `response.success` is true. */
	let recipe: Recipe = response.success ? response.data : (undefined as unknown as Recipe);

	let error: string[] = [];
	if (!response.success) {
		if (response.error?.message) error.push(response.error.message);
		if (response.error?.data?.message) error.push(response.error.data.message);
		if (response.error?.data?.data) error.push(JSON.stringify(response.error.data.data));
	}
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="The Nutriblocks Recipe App" />
</svelte:head>

<section>
	{#if !response.success}
		<!-- Could not get the recipe for whatever reason. -->
		<PageError>
			{error.join(': ') ?? "... and we don't know what it was 😱"}
		</PageError>
	{:else}
		<RecipeHeader recipeFn={() => Promise.resolve(recipe)} />

		<div class="w-full grid place-items-center my-12">
			<div class="w-[480px]">
				<Ingredients {recipe} />
			</div>
		</div>

		<div class="w-[1000px] mx-auto my-12">
			<Method {recipe} />
		</div>
	{/if}
</section>
