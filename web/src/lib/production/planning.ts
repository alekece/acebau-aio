export type RecipeRequirement = {
	pieceId: string;
	filamentSupplyId: string;
	quantity: number;
};

export type PieceCapacity = { id: string; capacity: number };

export type PlannedRun = {
	pieceId: string;
	filamentSupplyId: string;
	quantity: number;
};

/** Expands product quantities into one row per printable build-plate run. */
export function expandRecipeIntoRuns(
	recipe: RecipeRequirement[],
	pieces: PieceCapacity[],
	productQuantity: number
): PlannedRun[] {
	if (!Number.isInteger(productQuantity) || productQuantity < 1) return [];

	return recipe.flatMap((requirement) => {
		const piece = pieces.find((candidate) => candidate.id === requirement.pieceId);
		const capacity = Math.max(1, Math.trunc(piece?.capacity ?? 1));
		let remaining = requirement.quantity * productQuantity;
		const runs: PlannedRun[] = [];

		while (remaining > 0) {
			const quantity = Math.min(capacity, remaining);
			runs.push({
				pieceId: requirement.pieceId,
				filamentSupplyId: requirement.filamentSupplyId,
				quantity
			});
			remaining -= quantity;
		}

		return runs;
	});
}
