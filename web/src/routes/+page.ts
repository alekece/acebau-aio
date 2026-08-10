import { Tone } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = () => ({
	actions: [
		{
			title: 'Accepter ou rejeter 2 demandes',
			detail: 'Bloom Bloom · Maison Dune',
			module: 'Commande',
			tone: Tone.Warning,
			icon: 'orders',
			href: '/orders'
		},
		{
			title: 'Relancer la facture A-2026-034',
			detail: '1 240 € restent à encaisser depuis 8 jours',
			module: 'Facture',
			tone: Tone.Error,
			icon: 'invoices',
			href: '/invoices'
		},
		{
			title: 'Lancer la production de 3 pièces',
			detail: 'Promesse client dans 4 jours · 11 h estimées',
			module: 'Production',
			tone: Tone.Secondary,
			icon: 'production',
			href: '/production'
		},
		{
			title: 'Réapprovisionner le filament blanc',
			detail: 'Stock prévisionnel sous le seuil dans 6 jours',
			module: 'Stock',
			tone: Tone.Warning,
			icon: 'inventory',
			href: '/inventory'
		},
		{
			title: 'Déclarer la TVA de juillet',
			detail: 'Échéance le 19 août · montant à confirmer',
			module: 'Finance',
			tone: Tone.Warning,
			icon: 'finance',
			href: '/finance'
		},
		{
			title: 'Relancer le suivi de Maison Dune',
			detail: 'Dernier contact il y a 12 jours',
			module: 'CRM',
			tone: Tone.Surface,
			icon: 'resellers',
			href: '/resellers'
		}
	],
	summaries: [
		{
			name: 'Commandes',
			value: '2 demandes à traiter',
			detail: '842 € HT à accepter ou rejeter',
			action: 'Ouvrir les commandes',
			tone: Tone.Warning,
			icon: 'orders',
			href: '/orders'
		},
		{
			name: 'Production',
			value: '11 tâches en cours',
			detail: '2 promesses menacées · 91 % de réussite',
			action: 'Ouvrir la production',
			tone: Tone.Secondary,
			icon: 'production',
			href: '/production'
		},
		{
			name: 'Inventaire',
			value: '6 articles sous seuil',
			detail: '1 production bloquée · 24 bobines',
			action: 'Ouvrir l’inventaire',
			tone: Tone.Warning,
			icon: 'inventory',
			href: '/inventory'
		},
		{
			name: 'Factures',
			value: '6 380 € à encaisser',
			detail: '1 240 € en retard',
			action: 'Ouvrir les factures',
			tone: Tone.Error,
			icon: 'invoices',
			href: '/invoices'
		},
		{
			name: 'Finance',
			value: '9 629 € disponibles',
			detail: 'Trésorerie prudente après provisions',
			action: 'Ouvrir la finance',
			tone: Tone.Success,
			icon: 'finance',
			href: '/finance'
		},
		{
			name: 'Revendeurs',
			value: '6 actifs',
			detail: '5 relances prévues · 18 commandes sur 6 mois',
			action: 'Ouvrir les revendeurs',
			tone: Tone.Success,
			icon: 'resellers',
			href: '/resellers'
		}
	],
	workshop: [
		{
			name: 'Atlas · impression',
			detail: 'Pied ONDRA · fin dans 1 h 42',
			state: 'Production',
			tone: Tone.Secondary
		},
		{
			name: 'Mistral',
			detail: 'Disponible · prochaine maintenance à planifier',
			state: 'Disponible',
			tone: Tone.Success
		},
		{
			name: 'K2 du fond',
			detail: 'Maintenance · contrôle du plateau requis',
			state: 'Maintenance',
			tone: Tone.Warning
		}
	]
});
