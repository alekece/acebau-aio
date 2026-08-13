import type { Action } from 'svelte/action';

type InvalidShakeParameters = {
	invalid: boolean;
	attempt: number;
};

export const invalidShake: Action<HTMLElement, InvalidShakeParameters> = (node, parameters) => {
	let animatedAttempt = -1;
	let animation: Animation | undefined;

	function update({ invalid, attempt }: InvalidShakeParameters) {
		if (!invalid) {
			animation?.cancel();
			return;
		}
		if (attempt === animatedAttempt) return;

		animatedAttempt = attempt;
		animation?.cancel();
		if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) return;
		animation = node.animate(
			[
				{ transform: 'translateX(0)' },
				{ transform: 'translateX(-3px)' },
				{ transform: 'translateX(3px)' },
				{ transform: 'translateX(-2px)' },
				{ transform: 'translateX(0)' }
			],
			{ duration: 240, easing: 'ease-out' }
		);
	}

	update(parameters);

	return {
		update,
		destroy: () => animation?.cancel()
	};
};
