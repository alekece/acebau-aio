import { emptyPage, graphqlOrFallback, type Page } from '$lib/api/graphql';
import { Tone } from '$lib/components/ui/presets';
import type { PageLoad } from './$types';

type ProductRecord = {
	id: string;
	status: string;
	name: string;
	collection: string;
	category: string;
	shortDescription: string;
	customizable: boolean;
};
type VariantRecord = {
	id: string;
	productId: string;
	displayName: string;
	sku: string;
	retailPrice: string;
	status: string;
};
type CatalogueResult = { products: Page<ProductRecord>; variants: Page<VariantRecord> };

const fallback: CatalogueResult = {
	products: emptyPage([
		{
			id: 'demo-ondra',
			status: 'active',
			name: 'ONDRA',
			collection: 'Collection 2026',
			category: 'Éclairage',
			shortDescription: '',
			customizable: false
		},
		{
			id: 'demo-bloom',
			status: 'active',
			name: 'Bloom',
			collection: 'Collection 2025',
			category: 'Luminaire',
			shortDescription: '',
			customizable: false
		},
		{
			id: 'demo-mistral',
			status: 'draft',
			name: 'Mistral',
			collection: 'Collection 2025',
			category: 'Objet décoratif',
			shortDescription: '',
			customizable: false
		}
	]),
	variants: emptyPage([
		{
			id: 'demo-ondra-s',
			productId: 'demo-ondra',
			displayName: 'ONDRA · S',
			sku: 'OND-S-BL',
			retailPrice: '89 €',
			status: 'active'
		},
		{
			id: 'demo-ondra-m',
			productId: 'demo-ondra',
			displayName: 'ONDRA · M',
			sku: 'OND-M-NR',
			retailPrice: '109 €',
			status: 'active'
		},
		{
			id: 'demo-ondra-l',
			productId: 'demo-ondra',
			displayName: 'ONDRA · L',
			sku: 'OND-L-SG',
			retailPrice: '129 €',
			status: 'active'
		},
		{
			id: 'demo-bloom-white',
			productId: 'demo-bloom',
			displayName: 'Bloom · blanc',
			sku: 'BLM-BL',
			retailPrice: '74 €',
			status: 'active'
		},
		{
			id: 'demo-bloom-sand',
			productId: 'demo-bloom',
			displayName: 'Bloom · sable',
			sku: 'BLM-SA',
			retailPrice: '82 €',
			status: 'active'
		}
	])
};

export const load: PageLoad = async ({ fetch, parent, url }) => {
	const { defaultPageSize } = await parent();
	const page = Number(url.searchParams.get('page') ?? 1);
	const result = await graphqlOrFallback<CatalogueResult>(
		fetch,
		`query CataloguePage($page: Int!, $pageSize: Int!) {
		products(page: $page, pageSize: $pageSize) {
			items { id status name collection category shortDescription customizable }
			page pageSize totalItems totalPages
		}
		variants(pageSize: 100) { items { id productId displayName sku retailPrice status } }
	}`,
		fallback,
		{ page, pageSize: defaultPageSize }
	);

	return {
		products: result.value.products.items.map((product) => {
			const variants = result.value.variants.items.filter(
				(variant) => variant.productId === product.id
			);
			return {
				id: product.id,
				name: product.name,
				collection: product.collection,
				categoryName: product.category,
				shortDescription: product.shortDescription,
				customizable: product.customizable,
				category: `${product.category} · ${product.collection}`,
				variants: `${variants.length} variante${variants.length > 1 ? 's' : ''}`,
				price: variants.length
					? variants.map((variant) => variant.retailPrice).join(' – ')
					: 'À définir',
				sales: '—',
				status:
					product.status === 'active'
						? 'Actif'
						: product.status === 'draft'
							? 'Brouillon'
							: 'Archivé',
				tone: product.status === 'active' ? Tone.Success : Tone.Warning,
				alert: variants.length ? '—' : 'Première variante à créer',
				children: variants.map((variant) => ({
					id: variant.id,
					name: variant.displayName,
					sku: variant.sku,
					price: variant.retailPrice,
					margin: 'Marge à calculer'
				}))
			};
		}),
		kpis: {
			activeProducts: result.value.products.items.filter((product) => product.status === 'active')
				.length,
			activeVariants: result.value.variants.items.filter((variant) => variant.status === 'active')
				.length,
			belowMargin: 0
		},
		pagination: result.value.products,
		marginChannels: [
			{ label: 'Revendeur', target: '40 %', count: 0, tone: Tone.Warning },
			{ label: 'Site', target: '60 %', count: 0, tone: Tone.Warning },
			{ label: 'Direct', target: '80 %', count: 0, tone: Tone.Error }
		],
		usingFallback: result.usingFallback,
		loadError: result.error
	};
};
