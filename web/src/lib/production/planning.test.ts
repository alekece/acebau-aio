import { describe, expect, it } from 'vitest';
import { expandRecipeIntoRuns } from './planning';

describe('expandRecipeIntoRuns', () => {
	it('creates one vase run and four insert runs for four products', () => {
		const runs = expandRecipeIntoRuns(
			[
				{ pieceId: 'vase', filamentSupplyId: 'blue', quantity: 1 },
				{ pieceId: 'insert', filamentSupplyId: 'blue', quantity: 1 }
			],
			[
				{ id: 'vase', capacity: 4 },
				{ id: 'insert', capacity: 1 }
			],
			4
		);

		expect(runs).toEqual([
			{ pieceId: 'vase', filamentSupplyId: 'blue', quantity: 4 },
			{ pieceId: 'insert', filamentSupplyId: 'blue', quantity: 1 },
			{ pieceId: 'insert', filamentSupplyId: 'blue', quantity: 1 },
			{ pieceId: 'insert', filamentSupplyId: 'blue', quantity: 1 },
			{ pieceId: 'insert', filamentSupplyId: 'blue', quantity: 1 }
		]);
	});
});
