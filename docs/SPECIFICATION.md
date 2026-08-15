# Acebau all-in-one — MVP product specification

**Version:** 1.9  
**Date:** 10 August 2026  
**Status:** Final MVP scope, ready for autonomous coding-agent implementation  
**Owner:** Acebau

This document is the functional source of truth for the repository implementation and is written to be executed directly by a coding agent (e.g. Codex) with minimal external clarification. Earlier interactive previews remain visual references only: when a preview conflicts with this specification, this document prevails. Frontend and backend work must be derived from the behaviours, invariants and acceptance criteria defined below rather than from prototype-only data or session state.

**How an implementing agent should read this document:** every numbered requirement and every acceptance-criteria item in Sections 14 and 18 is a discrete, independently verifiable unit of work. Do not infer unstated behaviour; where this document is silent, prefer the narrowest interpretation consistent with Section 2 (MVP boundaries) and flag the gap in the pull request description rather than guessing. Section 18 ("Instructions for the implementing coding agent") is the authoritative entry point for tooling, stack, environment and workflow decisions and must be read before any code is written.

## 1. Product vision

Acebau needs one dedicated application to replace repeated manual work across spreadsheets and documents. The MVP must support one complete operational loop:

1. Define products, variants, recipes, costs and prices.
2. Receive or enter an order.
3. Reserve finished stock and identify what must be produced.
4. Plan and record production.
5. Consume supplies and add finished products to stock.
6. Prepare and ship the order.
7. Import the authoritative invoice and record payment.
8. Follow revenue, expenses, VAT and URSSAF actions.

The MVP is not intended to automate the whole workshop. It centralises reliable data, makes costs and stock traceable, and guides the administrator while leaving operational decisions under their control.

## 2. MVP boundaries

### 2.1 Included

- A publicly deployed application, reachable over the public internet at a production URL (not restricted to a local machine or private network).
- One administrator role. The internal application (every module described in Sections 5–13) is gated behind a mandatory login page (Section 3.1.1); there is no unauthenticated path into internal modules.
- A restricted public catalogue for approved professional resellers, reachable without login through a private, unguessable link combined with a short e-mailed access code that identifies the reseller (Section 8.2).
- A French/English interface selector in the global header. French is the complete reference interface for the MVP; the navigation and shared interface architecture are ready for English. Product and commercial content is maintained only in French.
- Three selectable interface colour themes using the same component hierarchy and accessibility contrast rules.
- Responsive desktop and mobile layouts.
- Persistent relational data and file storage.
- Catalogue, costing, pricing, supplies, finished stock, production, machines, reseller CRM, orders, shipping, imported invoices, expenses, tax assistance, settings and action dashboard.
- Manual recreation of catalogue, machines, supplies and resellers.
- Import of existing historical revenue and expenses.
- Database backup/export and restore/import, including an automatic snapshot taken before every schema migration so a failed or unwanted migration can be rolled back without losing data (Section 3.3).

### 2.2 Explicitly postponed

- Multiple administrator accounts, roles/permissions and self-service account creation (Section 3.1 covers exactly one administrator credential in the MVP).
- Third-party/social sign-in (Google, Microsoft, etc.), multi-factor authentication and self-service password reset by e-mail (see Section 3.1.1 for the minimal MVP login behaviour and the manual reset procedure).
- Translated product and commercial content.
- Per-reseller pricing, delivery thresholds and catalogue visibility.
- Consignment stock management.
- Automatic invoice-platform, marketplace, bank and carrier synchronisation.
- Partial invoice payments and automatic accounting reconciliation.
- Automatic production optimisation and machine control.
- Detailed manual-capacity planning.
- Automatic supplier orders.
- Freelance project and time management.
- Configurable, user-defined report exports (backup/restore of the whole database is in scope — see Section 3.3 — but ad hoc exports of arbitrary filtered views are not).

The database should remain extensible for these features, but the MVP interface must not expose incomplete controls for them.

## 3. Users and access

### 3.1 Administrator

The administrator has full access to every internal module. Because the MVP is deployed on the public internet rather than a local or private network, access to every internal module is protected by mandatory authentication instead of network placement.

#### 3.1.1 Login page

- The login page is the only unauthenticated route that exposes a form; every other internal route (all pages under Sections 5–13, including the dashboard) redirects an unauthenticated visitor to the login page and returns them to the page they originally requested after a successful login.
- The MVP recognises exactly one administrator credential: an e-mail address and a password, provisioned at deployment time (seed/environment configuration), not through a public sign-up form. There is no self-service registration route in the MVP.
- Fields: e-mail and password, both required. Password is entered in a masked field with a visibility toggle.
- Submitting invalid credentials shows a single generic error (`E-mail ou mot de passe incorrect`) that does not reveal whether the e-mail exists, and preserves the entered e-mail but never the password.
- Failed attempts are rate-limited per account and per source IP (see Section 18.6 for concrete thresholds); exceeding the limit temporarily blocks further attempts and shows a clear retry-after message rather than a silent failure.
- On success, the server issues a session (Section 18.6). The client stores no long-lived credential itself.
- A `Se souvenir de moi` checkbox extends session lifetime; without it, the session expires at the shorter default lifetime defined in Section 18.6.
- A visible `Se déconnecter` control is available from every internal page (top bar) and immediately invalidates the session server-side, not only client-side.
- Password reset in the MVP is a manual, administrator-triggered procedure (documented in Section 18.6); there is no automated "forgot password" e-mail flow in the MVP.
- The login page and every internal page are served exclusively over HTTPS; HTTP requests are redirected to HTTPS.

### 3.2 Approved reseller

An approved reseller receives a private, revocable catalogue link together with a short access code (Section 8.2) sent by e-mail; both are required to open the catalogue. No account or password is required. The reseller supplies contact and delivery details when submitting an order request.

The reseller can see only:

- Active catalogue products and variants.
- French product information and galleries.
- Their global reseller purchase price HT.
- Recommended retail price TTC.
- Made-to-order availability.
- Estimated dispatch range.
- Their current basket and submitted request confirmation.

They cannot see internal costs, margins, supplies, stock quantities, machine load, other resellers or internal order notes.

### 3.3 Backup and restore

Losing production, order, stock or finance data is treated as a serious incident, so the MVP includes basic, administrator-only backup and restore:

- **Manual export:** from Settings, the administrator can trigger a full database backup (data + file-storage references) and download it as a single archive on demand.
- **Automatic pre-migration snapshot:** every time a database migration runs (Section 18.2), the deployment process takes a full snapshot of the current database immediately before applying the migration. The snapshot is retained and identified by timestamp and the migration version it precedes, so a failed or unwanted migration can be rolled back without manual intervention.
- **Restore:** the administrator can restore the database from a previous snapshot or a manually uploaded backup archive. Restoring is a destructive, confirmed action that replaces current data; the interface requires an explicit confirmation step and states clearly which snapshot will be restored and when it was taken.
- **Retention:** the MVP keeps at least the last few automatic pre-migration snapshots and all manual exports until the administrator deletes them; unlimited automatic retention and off-site/scheduled backup policies are not required for the MVP but the storage design should not prevent adding them later.
- Backup files are stored using the same file-storage abstraction as other files (Section 12.3) and are never publicly reachable — only the authenticated administrator session can trigger, list or download them.

## 4. Global interface rules

### 4.1 Navigation

The internal primary navigation is grouped by the user’s mental model:

- **Pilotage:** Tableau de bord.
- **Atelier:** Production, Machines, Inventaire.
- **Offre:** Catalogue, Pièces 3D.
- **Ventes:** Commandes, Factures, Revendeurs.
- **Analyse:** Finance, Données.
- **Configuration:** Paramètres.

Desktop uses a persistent sidebar. Mobile uses a labelled menu button, a dismissible navigation drawer and a backdrop. The current page is identifiable without relying on colour. A global search entry point is always available and can be opened with `Ctrl/Cmd + K`.

The sidebar brand displays `ACEBAU` without an `Atelier` subtitle. The top bar contains global search, colour-theme selection, interface-language selection and notifications. It does not contain an “Accès rapide” page selector.

### 4.2 Visual and interaction principles

**Visual language.** The interface should read as a modern, contemporary product rather than a dense legacy back-office tool:

- Generous white space and clear visual hierarchy over tightly packed, dense screens; group related fields and actions with breathing room rather than borders alone.
- A restrained, contemporary sans-serif type scale with clear size/weight steps between page titles, section headers, body text and captions.
- Soft elevation (subtle shadow or a light border) to separate cards, panels and dialogs from the page background, rather than heavy boxed borders everywhere.
- Rounded corners and a consistent, moderate corner radius across cards, buttons, inputs and dialogs.
- Colour used sparingly and purposefully: a neutral base palette with accent colours reserved for primary actions, status and the three selectable colour themes (Section 2.1); no large flat blocks of saturated colour purely for decoration.
- Smooth, subtle motion (hover, focus, panel open/close, loading) that confirms an interaction happened without slowing the user down or being distracting; motion always respects reduced-motion accessibility preferences.
- These preferences apply consistently across all three colour themes and both desktop and mobile layouts; they must never come at the cost of the accessibility and contrast rules already required elsewhere in this section.

- Default body text should be at least 15–16 px.
- Tables use the same typography, spacing and status conventions everywhere.
- Recognisable icons are 20–24 px and paired with labels or tooltips.
- Primary actions use explicit labels such as `Créer une production`, not ambiguous icons alone.
- Destructive actions require confirmation.
- Status must never be communicated by colour alone.
- Forms preserve entered values when an inline item is created.
- Important totals and warnings remain visible on long forms.
- Mobile screens replace dense tables with cards and preserve the primary action in a sticky footer where useful.
- Keyboard focus is visible on every interactive control. A skip link lets keyboard users move directly to the main content.
- Mobile touch targets are at least 44 × 44 px whenever the layout permits.
- Long detail pages provide a sticky section navigation so the user can move between their operational sections without losing context.
- Empty states explain why no result is visible and provide the most relevant next action.
- The small contextual line above a page title is omitted on ordinary module pages. It is retained only when it identifies useful record context, such as product, order, reseller or performance scope.

### 4.3 Shared page behaviour

List pages support:

- Search.
- Sort.
- Relevant filters and status tabs.
- Pagination.
- Loading, empty and error states.
- Row expansion or a dedicated detail page.
- Contextual row actions.

Tables display a configurable number of parent records per page — 10 by default — set globally in Settings (Section 11.13) and adjustable per table by the administrator (e.g. 10/25/50/100) for that session or view. Clicking a row opens its detail when opening is the only available action; in that case there is no redundant `Open` button or action column. A final action column is present only when a record exposes additional contextual actions such as edit, download, remind, archive or delete. Expandable child content does not consume the page-size limit.

Tabs expose their selected state programmatically and keyboard users can reach each option. On mobile, each table row becomes a labelled card and the user keeps access to a compact sort selector. Search and filters update the visible result count and can be reset together.

Dialogs:

- Have an explicit title and action label.
- Place initial focus inside the dialog, trap focus while open and restore focus to the triggering control when closed.
- Close with `Escape` unless a blocking transaction is being confirmed.
- Validate required values inline and preserve all valid values after an error.
- Use one clear close action for informational content and visually distinguish destructive confirmation.

Every state-changing action gives immediate feedback. Unsaved settings display a persistent dirty-state indicator until saved. Temporary success feedback is announced without interrupting the current task.

No table export is required in the MVP.

### 4.4 KPI design rules

KPIs exist to support a decision, expose an exception or show progress toward an operational objective. A page should not display a number simply because it is available.

- Prefer actionable quantities such as `2 orders at risk` over descriptive totals such as `8 open orders`.
- Pair every warning KPI with the affected value, deadline or recommended next action.
- Use a clear period and comparison basis, for example `last 30 days` or `versus 2025`.
- Avoid duplicating the same KPI on the dashboard and a dedicated page unless the dashboard value is required for an immediate cross-module decision.
- Keep historical performance on the Data page and immediate operational exceptions on operational pages.
- The dashboard remains an action centre and contains only a small cross-module overview.
- A count needs no `(nb)` suffix: `Commandes à traiter` is already unambiguous.
- Percentages and monetary or physical units stay with the value, for example `82 %`, `7 180 €`, `18 h` or `3,8 kg`.
- Parentheses in a KPI label identify only a period or comparison window, for example `Taux de réussite (30 j)`.
- A contextual sentence may combine the measure and period, for example `Charge machine — 82 % sur les 7 derniers jours`.
- KPI cards are never interactive. Navigation and actions belong to adjacent lists, summaries or explicit buttons. A small `?` control may expose the KPI definition, period or calculation basis in a tooltip.

### 4.5 Notifications

Internal alerts are in-app only. Each alert has a severity, creation date, optional due date, target link, read state and optional snooze. Alerts should resolve automatically when the underlying issue is corrected where possible.

External transactional emails are allowed for reseller request receipt, acceptance, rejection and shipment updates.

## 5. Catalogue, costing and pricing

### 5.1 Data hierarchy

`Collection → Category → Product → Variant → Recipe`

- A product groups all its variants.
- No main or default variant is created automatically.
- A variant may be created from scratch or copied from another variant of the same product.
- Copying opens a selection dialog. Recipe, attributes, media and operational settings are selected by default.
- SKU, stock, orders, sales and production history are never copied.

### 5.2 Product

Required fields:

- French name.
- Collection.
- Category.
- Status: Draft, Active or Archived.

The editor preselects `Active`. Choosing `Draft` is a deliberate manual action; if activation requirements are missing, the application explains what is blocking activation and lets the administrator save the record as Draft.

Optional fields:

- French short and long descriptions.
- Care or usage information.
- Tags.
- Customisation availability.

### 5.3 Variant

A variant contains:

- Product reference.
- French display name.
- Free-form attribute/value pairs, for example `Couleur: Sable`, `Taille: M`.
- Attribute values may be suggested from previous entries but remain independent per variant.
- SKU.
- Status.
- Variant gallery.
- Recipe.
- Retail TTC price and reseller HT price.

The product gallery is the merged gallery of its active variants.

### 5.4 Activation

A variant can become Active only when the information required to cost and produce it is present:

- Unique SKU.
- At least one recipe component.
- Valid quantities and units.
- Complete printed-piece machine data where relevant.
- Valid labour and pricing configuration.

Missing descriptions or images generate warnings but do not block activation.

### 5.5 SKU

The generator is hard-coded as `{collection:3}-{category:4}-({attribute:2}-*)`:

- First three letters of the collection.
- First four letters of the category.
- Two-character codes for every non-empty variant attribute, in a stable order.

For example, a variant with size, colour and edition attributes can generate `OND-LAMP-M-BL-OR`. Colour and size are the only suggested attributes in the MVP, but other free-form attributes remain possible.

The generated SKU can be edited before its first operational use. It becomes immutable after being referenced by stock, production or an order. Every SKU must be unique.

### 5.6 Recipe

A variant recipe contains:

- Printed pieces and required quantity.
- Filament selected independently for every printed piece.
- Production materials and quantities.
- Product-packaging supplies and quantities.
- One total manual labour duration for assembly, cleaning and product packaging.

Machine compatibility, preference, exclusion, time and consumption belong to the printed-piece machine profile, not to the variant recipe.

Every printed-piece definition belongs to the reusable piece library. A piece may currently be referenced by only one product, but it has no `reusable` or `product-specific` scope. Pieces, materials and packaging can be selected from existing records or created inline without losing the current form.

Shipping packaging is not part of the product recipe and is consumed during shipment.

### 5.7 Printed-piece machine profile

For each compatible piece, machine model and nozzle diameter:

- Printing time for one piece.
- Estimated filament consumption for one piece, expressed as mass only. Material, colour and physical spool are not selected in the machine profile; the material/colour belongs to the variant recipe and the physical spool is selected during production.
- Maximum accepted pieces on one build plate.
- Quality rating or note.
- Preferred-machine-model flag.
- Excluded-machine-model flag.

If several pieces are printed on a plate, time and filament are estimated by multiplying the per-piece values by the number of pieces. The number of print runs is `ceil(required pieces / build-plate capacity)`.

Printed pieces are also managed from a dedicated library page. The administrator can search, create, edit, duplicate, archive or delete a piece. Deletion is allowed only when the piece has never been referenced by a recipe, production snapshot or historical record. Otherwise, the application offers archiving. The page must expose missing machine profiles and high-failure pieces.

### 5.8 Cost formulas

Filament cost:

`filament grams ÷ 1000 × weighted-average cost/kg × waste factor`

Machine cost:

`printing hours × machine hourly cost`

Labour cost:

`manual minutes ÷ 60 × labour hourly rate`

Variant base cost:

`printed pieces + production materials + product packaging + labour`

Channel cost adds the relevant selling fees. Percentage fees are applied to the selling price; fixed fees are added per sale. Annual or monthly subscriptions are represented by an empirical overhead percentage for margin estimation while the real subscription remains a Finance expense.

### 5.9 Machine usage and production-equipment cost

Default machine usage rate:

`(purchase + expected lifetime maintenance) ÷ expected lifetime printing hours`

Purchase cost includes setup cost. Expected lifetime printing hours are the model lifetime converted to hours and multiplied by a 50% default utilisation rate. For example, continuous operation every hour of every day would be 100% utilisation.

Electricity remains a separate production-environment cost:

`average electricity consumption × electricity rate`

The production-equipment cost is the machine usage rate plus the electricity cost. The calculated machine usage rate can be overridden per machine. Both calculated and effective values remain visible when an override exists.

### 5.10 Selling-price suggestion

Global initial gross-margin targets, based on selling price:

- Direct: 80%.
- Website: 60%.
- Reseller: 40%.

For retail HT price `P` and reseller divider `D`, reseller HT price is `P ÷ D`.

The application calculates the minimum retail HT price required by each channel and proposes:

`max(direct requirement, website requirement, D × reseller requirement)`

It then applies VAT and rounds upward using configurable psychological endings, for example €99 or €189. The administrator can accept the proposal or enter a final price manually. All channel margins remain visible, with a warning when the manual price misses a target.

Consumer retail prices are displayed TTC. Professional reseller purchase prices are displayed HT.

## 6. Supplies and finished inventory

### 6.1 Supply inventory

Tracked types:

- Individual filament spools.
- Production materials.
- Product packaging.
- Shipping packaging.

The MVP has one location: Atelier.

Each generic supply has a base unit such as gram, piece, metre or millilitre. A material can have several supplier offers with supplier reference, package quantity, package price and conversion to the base unit.

### 6.2 Filament spools

Every physical spool is tracked separately because nominally identical colours can produce visible differences. A spool records:

- Internal reference.
- Material and exact colour.
- Supplier and receipt.
- Initial and remaining weight.
- Weighted or received cost/kg.
- Optional lot reference.
- Opened date.
- Quality notes.
- Physical status: Sealed, Open, Empty or Discarded.
- Planning sufficiency, derived for the selected print: Open with enough material or Open without enough material.

One print job must use one spool with enough remaining material for the complete uninterrupted print. The application prefers a compatible open spool with sufficient material. Two insufficient spools must not be combined for one print.

Remaining weight is calculated from recorded consumption and can be corrected after manual weighing.

### 6.3 Receipts

A receipt may be created independently when goods arrive or offered from an expense. Confirming a receipt increases stock and updates weighted-average cost:

`(existing value + received value) ÷ (existing quantity + received quantity)`

### 6.4 Finished stock

For every variant and condition, show:

- Physical quantity.
- Reserved quantity.
- Available quantity = physical − reserved.

Production completion increases physical stock. Order acceptance reserves available stock. Shipment decreases physical and reserved stock. Rejection or cancellation releases reservations. Stock cannot become negative.

### 6.5 Défaut condition

A produced unit with a cosmetic or minor defect can enter finished stock with condition `Défaut`. Record the variant, quantity, description and optional photo.

Defective stock is never allocated automatically. The administrator must select it explicitly for an order. It can later be corrected to standard stock or scrapped through an adjustment.

### 6.6 Counts and corrections

- Complete count: freezes a count session, records counted quantities and creates justified differences on confirmation.
- Quick correction: adjusts one item with a required reason.
- Neither workflow permits negative stock.

### 6.7 Reordering

Every supply may define a low-stock threshold, target quantity and preferred supplier. When available quantity crosses the threshold, show the suggested quantity needed to reach the target. The MVP does not create purchase orders.

## 7. Production and machines

### 7.1 Production task

A task is created manually or from the production prompt of an accepted order. Nothing is created automatically without the administrator confirming it.

Task fields:

- Variant and product quantity.
- Optional linked order.
- Deadline.
- Manual priority override.
- Planning preference: Quality, Cost or Speed.
- Status: À planifier, À lancer, En cours, Terminée, Échec or Annulée.
- Recipe snapshot.
- Required printed pieces.
- Planned and actual consumption.
- Good, defective and failed output.
- Planned and actual costs.

The recipe snapshot ensures later catalogue changes do not rewrite historical production.

### 7.2 Print jobs

A task expands into print jobs by piece and selected machine. Statuses:

- To print.
- Printing.
- Done.
- Failed.

The administrator changes every status manually. Starting a job reserves the selected spool quantity where applicable. Completing or failing it records actual consumption.

A failed print requires a reason, failed quantity and actual waste. Notes and photos are optional.

### 7.3 Completion

Completing production is one atomic operation:

1. Confirm actual material and product-packaging consumption.
2. Refuse negative supply stock and request reconciliation if quantities are insufficient.
3. Consume confirmed supplies.
4. Add standard and/or Défaut finished units.
5. Update task and linked order progress.

The recipe is proposed by default, but actual quantities remain editable before confirmation.

### 7.4 Machine states

- Available.
- Currently printing, derived from active jobs.
- Maintenance.
- Temporarily unavailable.
- Archived.

Machine records contain model, current nozzle diameter, purchase and cost information, compatible profiles, effective hourly cost, total recorded printing hours and maintenance notes.

### 7.5 Advisory selection

The planner filters out excluded or unavailable machines and can sort compatible suggestions by:

1. Best quality, default.
2. Lowest estimated cost.
3. Fastest completion.

It never starts a machine or commits a schedule automatically. The administrator makes the final choice.

### 7.6 Machine timeline calendar

The machine calendar is part of the MVP.

- Day and week views.
- One row per machine.
- Planned and current jobs.
- Busy and free periods.
- Maintenance and unavailable periods.
- Overnight printing.
- Conflict and unavailable-machine warnings.
- Drag-and-drop rescheduling.

Dragging a job only changes its proposed start, end and machine assignment after compatibility validation. It never starts or stops a machine.

The estimated safe duration applies a configurable global planning-safety margin for possible reprints. For example, a 100% margin doubles planned print time. Historical failure suggestions are postponed.

## 8. Resellers and public catalogue

### 8.1 Reseller record

- Business and legal identifiers.
- Relationship status: Prospect, Approved, Paused, Closed or Rejected.
- Primary contact and optional secondary contacts.
- Primary shop/delivery location and optional additional locations.
- Private catalogue-link and access-code status.
- Commercial notes and interaction history.
- Dated next-action reminders.
- Order and imported-invoice history.
- Optional payment-term override.

Current reseller stock is not tracked. Only purchase history is retained.

### 8.2 Public catalogue access

- Only Approved resellers receive a private catalogue link.
- Links are signed, revocable and regenerable.
- No reseller account or password.
- **Access code:** alongside the link, the administrator (or the system, on approval) sends the reseller a short access code — 4 characters, letters and/or digits (e.g. `A7K2`) — by e-mail. The reseller must enter this code once per session before the catalogue is shown, even when opening the correct link.
- The link identifies which catalogue/pricing to show; the code identifies and confirms *which reseller* is opening it, so a forwarded or leaked link alone is not enough to reach the catalogue.
- The code is tied to one reseller record and is regenerated whenever the link is regenerated; entering an incorrect code shows a generic error and is rate-limited (same principle as Section 18.6) to prevent brute-forcing a 4-character code.
- A successfully entered code is remembered for the reseller's current browser session only; it is asked again in a new session, consistent with there being no persistent reseller account.

### 8.3 Catalogue behaviour

The public catalogue offers:

- Collection and category navigation.
- Search.
- Filterable attributes.
- Product grid and detail pages.
- Variant selection and galleries.
- Reseller purchase price HT.
- Recommended retail price TTC.
- Made-to-order indication.
- Estimated dispatch-date range.
- Basket.
- Order-request form.

There is no minimum-order quantity or value.

### 8.4 Customisation

Only products marked customisable offer a request option. Allowed requests are limited to configured values for main or secondary colour, cable colour and size. A colour already used elsewhere in the Acebau catalogue may be requested even if it is not a standard variant of that product.

The price is displayed as `À confirmer`. Full bespoke products direct the reseller to contact Acebau.

## 9. Orders and shipping

### 9.1 Sources

- Reseller catalogue request.
- Manual direct order.
- Manual Shopify order.
- Manual Etsy order.

Automatic marketplace import is postponed.

### 9.2 Reseller request flow

1. The reseller submits contact, delivery and basket information.
2. The application stores an editable Pending request and notifies the administrator.
3. The administrator views it and chooses Accept or Reject.
4. Rejection may include an optional reason and creates no reservation or production.
5. Acceptance creates a confirmed order, reserves available standard stock and opens the production prompt.

The MVP request states are deliberately limited to Pending, Accepted and Rejected. Before the administrator accepts or rejects it, the reseller may reopen the Pending request, edit configured lines, quantities, contact or delivery location, and submit a new revision. The application updates the same request, keeps its revision history and prevents duplicate submission.

### 9.3 Production prompt

For every order line, show:

- Ordered quantity.
- Standard stock available and quantity proposed from stock.
- Missing quantity proposed for production.
- Required printed pieces for the proposed production.
- A checkbox to produce new units instead of using available stock.
- Planning preference: Quality, Cost or Speed.

Defective stock is never proposed automatically. Production tasks are created only after explicit confirmation.

### 9.4 Editing and locking

A confirmed order can be edited until an issued invoice is imported and confirmed. Importing the invoice locks commercial quantities and prices. Operational shipment and payment statuses remain updateable.

### 9.5 Delivery

The reseller initially sees `Frais de livraison calculés après préparation`.

Delivery is free when the order value reaches the global HT threshold. The MVP has no proximity-postcode or geographic-radius rule.

Otherwise the administrator weighs the prepared parcel and enters the actual charge before invoice preparation. The administrator's approval is sufficient; the standard reseller does not reconfirm the delivery fee.

### 9.6 Shipment

The MVP supports one shipment event per order. That shipment may contain several parcels, each with its own label or carrier receipt. The shipment records:

- Shipment date.
- Carrier.
- Tracking number.
- One or more parcels and each parcel’s weight.
- Delivery charged and actual carrier cost.
- Shipping packaging consumed.
- Imported label or carrier receipt.

Confirming the shipment consumes all finished units in the order and the shipping packaging used by every parcel. The reseller can receive a shipment email. There is no partially shipped order state in the MVP; several parcel labels do not represent several shipments.

An order is Completed only when all products are shipped and its invoices are paid.

### 9.7 Custom request quotation

For an allowed custom request, the administrator adapts the order-specific recipe, sets the final price, accepts the request and notifies the reseller with a quote (`devis`). The reseller can confirm or refuse the quote. A refusal closes and drops the order. The configuration remains order-specific in the MVP, with an optional later action to promote it to the catalogue.

Quote acceptance does not start production automatically. The administrator explicitly authorises production per order.

## 10. Imported invoices and finance

### 10.1 Authoritative invoicing platform

Acebau will use an approved external electronic-invoicing platform for both Acebau and freelance invoices. The platform is the authority for numbering, issuance and authoritative files.

The MVP does not generate, number or issue legally authoritative invoices. The selected provider is configured later without changing the internal data model.

### 10.2 Manual invoice import

Import supports an invoice or credit-note file with manually entered or confirmed metadata:

- External platform and identifier.
- Number.
- Issue and due dates.
- Customer.
- Activity: Acebau or Freelance.
- Type: invoice or credit note.
- Lines and HT/VAT/TTC totals.
- Payment status.
- Original file.
- Optional linked Acebau order.
- Optional credited-invoice link.

Duplicate number or external identifier triggers a blocking warning. Invoice values are authoritative after import. Original order values remain in history for reconciliation.

Suggested internal numbering configuration is `A-YYYY-NNN`, restarting yearly, but numbering remains controlled by the external platform.

### 10.3 Invoice states

- Imported.
- Unpaid.
- Paid.
- Overdue, derived from due date.
- Credited.

The MVP records a complete invoice as paid manually. Partial payments are postponed. Default payment terms are global, with an optional reseller override.

### 10.4 Finance activities

Finance covers the whole individual enterprise and classifies entries as:

- Acebau.
- Freelance.
- Shared.

Freelance invoices are imported without an internal project or order.

### 10.5 Expenses

Manual expense fields:

- Supplier and supplier-invoice attachment.
- Invoice, accounting and payment dates.
- Activity and category.
- Description.
- HT, VAT and TTC.
- Suggested deductible VAT, requiring confirmation.
- Payment status and method.
- Notes.

For a tracked-supply purchase, offer to create a linked stock receipt. The stock increases only when the receipt is confirmed.

### 10.6 Marketplace fees and subscriptions

In the MVP, Shopify, Etsy and SumUp fees are entered as linked expenses or payout adjustments. Automatic gross/fee/net decomposition and reconciliation are postponed.

Recurring subscriptions are real Finance expenses. A separate empirical overhead percentage may approximate their effect in product costing.

### 10.7 VAT monthly workflow

For a selected month, propose:

- VAT collected.
- Confirmed deductible VAT.
- Credit notes and carry-forward.
- Manual adjustments.
- Proposed amount due or credit.

States: Upcoming, Ready, Declared, Paid and Overdue. The administrator submits the official declaration outside the application, enters the actual declared amount and marks payment with optional proof.

### 10.8 URSSAF monthly workflow

Propose turnover separated into configured goods and service categories, apply configured rates and show an estimated declaration. The administrator submits externally, records actual values and marks payment. The application must not claim to provide an official declaration.

## 11. Page specifications

### 11.1 Tableau de bord

**Purpose:** show what needs attention now.

**Small executive overview:** current-month turnover, open orders, production load and critical-alert count.

**Main content:** prioritised action list for pending reseller requests, production delays, stock shortages, shipments, overdue invoices, VAT/URSSAF actions, maintenance and reseller follow-ups.

**Behaviour:** each priority identifies its source with a compact module tag such as `Commande`, `Facture`, `Production`, `Stock`, `Finance` or `CRM`. Priority rows and module summaries open their source module or record; KPI cards remain informative and non-interactive. Snooze is available where meaningful, and resolved actions disappear automatically. Dashboard totals use the same period and accounting basis as their dedicated pages.

Immediately below the KPIs, a compact module overview summarises Orders, Production, Inventory, Invoices, Finance and Resellers. Every summary contains one useful state, one exception or trend and a direct link; it does not duplicate the full dedicated page. Prioritised actions and the current workshop state follow this overview.

### 11.2 Production

**Decision KPIs:** tasks to start, machine load for the next seven days, threatened promises, 30-day success rate and 30-day waste in weight and cost.

**Views:** task queue, piece/job queue and machine timeline calendar.

**Primary actions:** create production, start/finish/cancel task, assign machine/spool, record failure, complete production and reschedule timeline job.

**Timeline behaviour:** previous/next week and day/week controls update the visible period. Machine unavailability, current jobs and proposed jobs remain visually distinct. Moving or rescheduling work changes only the proposed plan until the administrator confirms its status.

The task table uses only `À planifier`, `À lancer`, `En cours`, `Terminée`, `Échec` or `Annulée` as task states. A threatened deadline is an exception attached to the task, not a state. Each task expands to a nested table that lists grouped printed pieces, quantities, assigned machine, state, spool, supply blockage and per-piece progress. The nested table must read as part of the parent row, not as unrelated cards.

Failure incidents are filterable and use a table with machine, piece and/or order, failure type and date. Compact KPIs show the total count for each failure family. The incident component is paired with capacity/blockage information on desktop. The machine timeline remains full width.

A production-history table exposes completed tasks, good and Défaut outputs, waste, completion date and result. Historical productions cannot be deleted.

### 11.3 Machines

**Decision KPIs:** usable machines, seven-day load, failures per 100 printing hours over 30 days and maintenance due. Current machine states remain visible in the table and load visual rather than being repeated as separate KPI cards.

**Content:** one consistent machine table with model/name, current state, current job, seven-day load, hourly cost, recorded hours, next maintenance and row actions. Machine cards are not used on the list page.

**Detail:** a deliberately simple record with state, current work, seven-day load, effective cost, hours and next maintenance. Contextual actions allow state updates and maintenance planning. The list page also retains the seven-day visual load timeline for every named machine.

### 11.4 Catalogue

**Decision KPIs:** active-product count, active-variant count and variants below a channel margin target.

**Content:** expandable product rows with variant subrows, price range, status, sales/production counts and alerts.

**Primary actions:** create product, expand product variants, open product/variant details, duplicate, archive and conditionally delete. The catalogue header does not expose a global `Add variant` action; a variant is added from its product.

Selecting a margin warning opens the affected variants and the missing margin points, rather than a generic notification.

The margin component has one compact view per channel - reseller 40%, site 60% and direct 80% - and lists only variants below that channel target. Every product row expands to show its variants. A variant has a dedicated detail page with identity, SKU, price/margins, recipe, machine profiles, stock and commercial performance.

### 11.5 Product and variant editor

Wizard steps:

1. Product identity, with the option to create the first variant in the same flow.
2. Variant attributes and SKU. Colour and size are suggested; other attributes remain free-form.
3. Printed pieces, quantity per product and the filament assigned to each printed piece. Machine preferences and exclusions remain in the printed-piece profile.
4. Materials and packaging.
5. Labour and machines.
6. Costs, suggested price and channel margins.
7. Images and activation review.

The first variant is optional rather than automatic. Product creation can continue through the same assistant to create it in one operation. A later variant is created from the product detail. Inline supply or piece creation returns the user to the same wizard state.

The product detail includes a representative gallery, an interactive variant list, reference cost averages or ranges, stock/production actions and all three channel margins. The variant detail includes its own gallery and attributes, complete cost breakdown, tabular recipe, machine profiles, all three margins and exactly three stock quantities: available, to produce and Défaut.

### 11.6 Orders

**Decision KPIs:** open-order value, average acceptance-to-shipment delay, on-time delivery rate and custom requests awaiting pricing. Counts already visible in the lifecycle distribution are not duplicated as KPIs.

**Content:** status tabs and order table with source, customer, dates, value, stock/production progress and next action.

**Primary actions:** enter manual order, accept/reject request and open order.

The open-order distribution is visually distinct from KPI cards and may use a donut chart. It contains `Demandes à traiter`, `Acceptées`, `En production`, `À expédier` and `À encaisser`. The production stage must not be omitted. Each order row expands to a compact nested product table for quick content inspection.

### 11.7 Order detail

**Header:** source, customer, request status, deadline/range, value and next action.

**Sections:** editable lines and reservations, custom quote, production authorisation, one shipment with multiple parcels/labels, invoice link, files and event history.

**Lifecycle:** a visible stepper shows `Demande → Acceptation → Production → Expédition → Facture payée` and identifies the current step without relying on colour.

**Sticky actions:** `Rejeter` and `Accepter` while Pending; edit/create production while Accepted; prepare shipment; import/link invoice; mark completion conditions. Accepting immediately reserves available standard stock and replaces the decision actions with the next valid production actions. Rejecting closes the request and removes invalid actions.

Stock coverage and production are one continuous component: requested quantity, available Standard quantity, a clearly aligned `do not use stock` checkbox, resulting quantity to manufacture, planning priority and `Create productions`. Toggling the checkbox recalculates the manufacturing quantity immediately.

### 11.8 Invoices

**Decision KPIs:** imported invoices not yet linked to an order/activity, total still to collect, overdue amount and invoices due within seven days. The interface uses `Importées non liées`, not the ambiguous label `À rapprocher`.

**Content:** invoice table with customer, activity, number, dates, totals, order link, status and actions. Activity filters include all activities, Acebau and freelance.

**Primary actions:** import invoice, confirm data, link order, open the imported PDF in a preview, download original, mark paid and prepare overdue follow-ups. Credit-note management is postponed beyond the MVP.

An overdue line exposes `Relancer`. The reminder contains recipient, subject and editable message, can be copied, and can open the default e-mail application. Standard `mailto` links cannot reliably attach a local PDF, so the real implementation downloads the invoice and clearly tells the administrator when the mail application requires manual attachment.

### 11.9 Finance

**Decision KPIs:** HT turnover split by Acebau/freelance, amount collected, expenses, result before contributions and VAT/URSSAF provision for the globally selected period. The default period is the current month.

**Sections, in order:** period selector, KPIs, prudent available cash, activity split, receivables, compact monthly trend, declaration actions, expandable month table and annual comparison. A supplier-payables component is excluded because ordinary purchases are paid when ordered.

The global period selector supports current month, previous month, year to date and custom dates. It controls KPIs, cash, activity split, receivables and charts. The VAT/URSSAF component has a separate declaration-month selector defaulting to the latest closed month.

The month-by-month table can show all activities, Acebau only or freelance only. It includes collected VAT and deductible VAT. Each month is closed by default and expands to show the underlying revenue/expense entries and their justificatives. The yearly table uses the same consolidated definitions so years remain comparable.

**Primary actions:** add expense, import historical entries through the staged workflow, open the VAT or URSSAF month, inspect the proposed calculation and mark declaration/payment.

### 11.10 Resellers

**Decision KPIs:** active approved resellers, reseller turnover, reseller orders for the latest six months, average reseller order and dated commercial follow-ups. Inactive count is inferred from the active ratio and is not repeated as a separate KPI.

**Content:** reseller table with relationship status, calculated activity, last activity without forcing it to be labelled as an order or contact, turnover and next action.

**Primary actions:** add reseller, edit, record interaction, schedule follow-up and manage private link/access code.

### 11.11 Reseller detail

**Sections:** identity, relationship and calculated activity, contacts and locations, catalogue access, commercial timeline, reminders, orders and invoices. A yearly activity table shows order count, HT turnover, average order and evolution.

Clicking any reseller row opens this detail. The page supports recording an activity with a dated next action, adding primary or secondary contacts, adding locations, managing the private link and access code and opening commercial-history records.

### 11.12 Inventory

Tabs:

- Filaments.
- Materials. This single tab contains both production materials and packaging supplies.
- Products.
- Stock movements.
- Counts.

**KPIs:** articles below threshold, blocked production, predicted shortages, approximate stock value and Défaut-product count. Défaut count supports planning a dedicated website sale; it is not framed as an immediate decision.

The filament table shows the number of spools and their state. `Alerte sous` means the available aggregate quantity that triggers a reorder suggestion. Expanding a filament shows its physical spools; the planner prefers an open spool containing enough material for the entire piece.

Materials and packaging share the visible `Materials` tab. Each supply is one interchangeable stock item and expands to show supplier offers, price and lead time. Products use `Standard` or `Défaut` as their condition; `low` is never a condition. Quantity and reorder warnings are separate data.

**Primary actions:** receive supplies, add supply/spool, quick correction, start complete count and inspect reorder suggestions.

### 11.13 Settings

Sections:

- General: business name, currency, VAT, timezone, country, nine-digit SIREN and fourteen-digit SIRET. No workshop-address field.
- Catalogue: hard-coded SKU formula `{collection:3}-{category:4}-({attribute:2}-*)`, default Active state and only Colour/Size as suggested attributes.
- Costing/pricing: channel targets, reseller divider, fees, overhead and smart psychological rounding. Values near a round threshold can use a 9-ending (102 -> 99), while a natural price such as 120 stays 120.
- Production: default planning mode, identical daily start/end times, labour cost, filament preparation loss and clearly explained planning-safety margin.
- Machines: model defaults, electricity, lifetime and maintenance reminders. Lifetime remains overridable per machine.
- Inventory: fixed weighted-average cost explanation, spool states and reorder behaviour.
- Orders/delivery: global free-delivery threshold and dispatch wording. No nearby-postcode list and no user-configurable shipment-count option; the MVP uses one dispatch with any required number of parcel labels.
- Resellers: private-link expiry defaults to `Never`, active-period rule, payment terms and access-code length/format.
- Finance: activities, URSSAF rates, VAT behaviour and invoice platform.
- Data: trigger a manual backup export, list available automatic pre-migration snapshots and manual exports with their date/size, and restore from a selected backup (Section 3.3).
- Display: default table page size (Section 4.3), applied across list pages unless overridden per table by the administrator.

### 11.14 Public reseller catalogue

Pages:

- Catalogue home/list.
- Product detail and variant selection.
- Basket.
- Order-request form.
- Submission confirmation.

The public design should feel like Acebau rather than an internal ERP, while keeping wholesale price and ordering information explicit.

The default view is a compact professional quick-order list, not a consumer webshop. Each product row contains dependent variant selectors, quantity, reseller HT price, recommended public TTC price, made-to-order state and an inline add action. A secondary `Product sheets` view retains editorial descriptions, selected-variant gallery and professional information.

Variant selectors are dependent. After a reseller selects one attribute, every following selector shows only values belonging to compatible real variants. For example, selecting a size removes colours and cable combinations that do not exist for that size. A configured line added to the basket stores the exact variant SKU, attributes, price and quantity. That SKU remains the authoritative source when the line price and request totals are displayed or recalculated, preventing an attribute/price mismatch.

Commercial conditions are visible before ordering: no minimum order, free shipping from 250 € HT, made-to-order production with an estimated dispatch range, freedom to set the retail price and permitted customisation. Product/category/text filters work together and show the current result count. Quantity controls recalculate the HT subtotal and free-shipping progress immediately. The request summary calculates HT, VAT and TTC; submission keeps the request in a review state and never implies acceptance.

The selected variant controls its gallery, professional HT price, recommended retail TTC price and calculated made-to-order dispatch range on the product sheet. Basket lines remain editable. Custom lines are visually separate and excluded from financial totals until quoted. Contact and delivery choices use the reseller’s saved contacts and locations. After submission, a genuine confirmation view shows the request reference, revision and estimated dispatch range. The reseller can edit and resubmit the same pending request until the administrator accepts or rejects it.

### 11.15 Printed pieces

**Purpose:** maintain 3D definitions that are all reusable across variant recipes.

**Decision KPIs:** active pieces, missing machine profiles and pieces below the target success rate. A missing preferred machine is not a KPI because preference is only a quality hint. Failure cost belongs to Production, not this page.

**Content:** searchable table with piece name/reference, machine-profile coverage, reference or average printing time, average filament consumption, recent success rate and row actions. There is no reusable/product-specific scope field and no expanded profile sublist on the overview page.

The list shows a reference or average printing time and filament consumption. Machine profiles are edited from piece creation or editing, rather than expanded as a large child list in the overview. The profile contains a machine model, nozzle diameter, printing time, filament consumption, accepted pieces per build plate, preferred-machine-model selector and exclusion checkbox. Capacity always belongs to the piece-machine-model-nozzle profile. For example, an XL lampshade can have capacity 1 and K2 Plus preferred while smaller machine models are excluded. At most one compatible machine-model and nozzle profile may be preferred; an excluded profile cannot also be preferred.

**Primary actions:** create, edit, manage machine profiles, archive and conditionally delete. `New piece` is a split action with `Create blank` and `From an existing piece`; duplication is not a separate page-level action.

### 11.16 Data and commercial performance

**Purpose:** understand what sells, what generates revenue and margin, and what should be developed, repriced, reduced or archived.

**Filters:** period, year, channel, collection, category, product and variant. Filters combine, update all summaries and analyses consistently and can be reset in one action.

**KPIs:** units sold, product turnover, gross margin, largest revenue contributor and the most important underperformance alert for the selected period.

A lifetime communication strip uses internally consistent totals since the October 2024 launch, including objects sold, cumulative product turnover, reseller-order count and best-selling product.

**Required analyses:**

- Sales and turnover by month and year.
- Units, turnover and margin by product.
- Variant ranking and trend.
- Revenue and margin by category.
- Revenue and margin by sales channel.
- Top performers and underperformers.
- Clear comparisons with the previous equivalent period.
- A product-performance detail with month-by-month and year-by-year units/turnover, channel split and per-variant performance. The same detail is reachable from product, variant and Data views.

This page contains historical commercial analytics. Immediate stock, production, order and cash exceptions remain on their dedicated pages.

## 12. Core workflows and consistency rules

### 12.1 Atomic stock operations

The following must be transactional: production completion, order reservation, order rejection/cancellation release, shipment confirmation and stock-count confirmation. A failure must leave all related quantities unchanged.

### 12.2 History and audit

Important changes store timestamp, actor (`Administrator` or `System`), previous value where appropriate and source record. Operational records referenced historically should be archived rather than deleted.

### 12.3 File handling

Files are stored separately from relational records with metadata, original filename, type, size, upload date and owning record. Supported examples include product images, defect photos, supplier invoices, shipping labels, imported invoices and declaration proof.

### 12.4 Dates and currency

- Currency: EUR.
- Timezone: Europe/Paris.
- Internal date/time values should be stored consistently and displayed in French local format.
- HT, VAT and TTC values retain sufficient decimal precision; displayed totals round according to currency rules.

## 13. Data migration

### 13.1 Manually recreated

- Catalogue and recipes.
- Machines and compatibility profiles.
- Supplies, supplier offers and filament spools.
- Resellers and prospects.

### 13.2 Opening inventory

Existing stock spreadsheets are not imported. Supply and finished stock start at zero. The administrator performs an opening physical count, and the confirmed count creates opening movements.

### 13.3 Historical finance

All existing revenue and expense entries are imported through a staging workflow:

1. Upload or enter source rows.
2. Map activity, category, dates and amounts.
3. Detect missing or duplicate values.
4. Preview totals against the source workbook.
5. Confirm import.

Historical entries do not create orders, catalogue items, stock movements or reseller records.

## 14. MVP acceptance criteria

The MVP is ready when the administrator can demonstrate the following without editing a spreadsheet:

1. Create ONDRA, add a colour/size variant and calculate its real cost.
2. Receive filament and materials and see weighted costs and spool quantities.
3. Open a reseller’s private catalogue and submit an order request.
4. Accept the request, reserve available products and create production for shortages.
5. Assign print jobs to machines and place them on the machine timeline.
6. Record a failed print, actual waste and a replacement print.
7. Complete production and obtain correct supply and finished-stock movements.
8. Record a Défaut unit without making it automatically available to standard orders.
9. Prepare and ship the complete order once, using one or more parcels and labels, and consume the correct reserved stock and shipping packaging.
10. Import and link the authoritative invoice and mark it paid.
11. Enter an Acebau supply expense and create its linked receipt.
12. Import a freelance invoice without creating an order.
13. Review proposed monthly VAT and URSSAF values and record declaration/payment.
14. See actionable alerts and the correct KPIs on each dedicated page.
15. Use the principal operational actions on both desktop and mobile.
16. Find a reusable printed piece, edit its machine profiles and archive it when deletion is not allowed.
17. Compare monthly and yearly Acebau/freelance turnover from Finance and open a month’s entries.
18. Identify the best and worst products, variants, categories and channels for a selected commercial period.
19. Navigate every internal view with a keyboard, identify the current page and open/close a dialog without losing focus context.
20. Search, filter, sort and open list records on desktop and mobile; mobile table cards retain their field labels and sorting control.
21. Accept or reject a reseller request and see only the actions valid for the resulting order state.
22. Combine public-catalogue filters, modify basket quantities and obtain updated line, HT, VAT and TTC totals.
23. Modify a setting, see that changes are unsaved, save it and receive clear confirmation.
24. Complete the main action on each dashboard alert and decision callout without reaching a placeholder or dead control.
25. Create a product and its optional first variant through the complete seven-step assistant, then create a later variant from the product detail.
26. Select dependent public-catalogue attributes and see only compatible real variant combinations.
27. Submit a reseller request, reopen it while Pending, edit it and save a new revision without creating a duplicate request.
28. Open a reseller from the list, inspect the restored detail, add a contact or location, record a dated next action and manage the private link.
29. Attempt to open any internal URL directly while logged out and be redirected to the login page; log in with the administrator credential and land back on the originally requested page; log out and confirm the session is invalidated server-side.
30. Trigger repeated invalid login attempts and confirm the account is temporarily rate-limited with a clear message, without revealing whether the e-mail address exists.
31. Trigger a manual backup export, confirm a migration produces an automatic pre-migration snapshot, and restore the database from a selected backup after an explicit confirmation step.
32. Open an approved reseller's catalogue link, enter their e-mailed access code, and confirm that a wrong or missing code blocks access to the catalogue.
33. Change the default table page size in Settings and confirm list pages across the application reflect the new size.

## 15. Recommended implementation order

### Phase A — Reliable product and stock data

Foundation, settings, catalogue, recipes, costs, prices, supplies, spool tracking, receipts, finished stock and counts.

### Phase B — Workshop execution

Machines, print profiles, production tasks, print jobs, failure/defect recording, atomic completion and machine timeline calendar.

### Phase C — Selling and fulfilment

Resellers, private catalogue, request approval, reservations, production prompt, manual orders and shipment.

### Phase D — Financial closure

Invoice import/linking, expenses, historical finance import, VAT/URSSAF assistance, action dashboard, responsive polish and end-to-end acceptance tests.

Every phase is part of the MVP. The phases are implementation checkpoints, not separate product editions.

## 16. Repository implementation handoff

### 16.1 Source-of-truth hierarchy

The implementation should use the following order of authority:

1. Domain invariants and acceptance criteria in this specification.
2. Page and workflow behaviours in this specification.
3. Approved visual patterns from the v1.5 prototype.
4. Illustrative prototype data.

Prototype values, temporary session state and placeholder calculations are never authoritative business data.

### 16.2 Frontend responsibilities

The frontend is responsible for presentation and interaction, including:

- Responsive desktop and mobile layouts.
- Accessible navigation, dialogs, tables, forms and feedback.
- Dependent variant selection and immediate display-only calculations.
- Optimistic interaction only when the backend can safely confirm or reject it.
- Preserving form drafts locally while a user creates an inline piece, material or supplier offer.
- Displaying backend-provided permissions, locks, warnings, totals and KPI definitions without reimplementing business rules independently.

The frontend must not be the sole authority for stock, costing, pricing, order locks, invoice reconciliation or workflow transitions.

### 16.3 Backend responsibilities

The backend is responsible for persistent domain rules, including:

- Catalogue activation and SKU uniqueness/immutability.
- Cost and channel-margin calculations.
- Weighted-average inventory costs and physical spool balances.
- Stock reservations, releases, consumptions and count adjustments.
- Production snapshots, print-job transitions, actual consumption and outputs.
- Request revisions, order acceptance/rejection and commercial locking.
- Shipment confirmation and packaging consumption.
- Invoice import, duplicate detection, linking and payment state.
- Finance totals, VAT/URSSAF proposals and KPI query definitions.
- File metadata, ownership, access and history.
- Audit events for every important state or quantity change.
- Authentication, session issuance/invalidation and login rate-limiting (Section 3.1.1 and Section 18.6). The frontend must never decide authentication state on its own; every internal request is authorised server-side.

Operations listed as atomic in section 12.1 must execute in a database transaction. Concurrent updates must not permit double reservation, negative stock, duplicate request acceptance or overwriting a newer request revision.

### 16.4 Suggested domain modules

The repository may organise code differently, but the following bounded modules should remain conceptually distinct:

- `catalogue`: collections, categories, products, variants, attributes, recipes, prices and galleries.
- `pieces`: printed pieces and machine-specific profiles.
- `machines`: machine records, states, hourly costs, maintenance and timeline availability.
- `inventory`: supplies, supplier offers, spools, receipts, finished stock, reservations, movements and counts.
- `production`: tasks, print jobs, scheduling suggestions, failures, waste and completion.
- `resellers`: organisations, contacts, locations, relationship/activity, follow-ups, private links and access codes.
- `orders`: public requests, revisions, manual orders, quotations, reservations, production authorisation and shipment.
- `invoices`: imported authoritative invoices, links, locks, due dates, reminders and payment.
- `finance`: activities, expenses, marketplace fees, historical entries, VAT, URSSAF and analytical totals.
- `analytics`: shared, documented commercial and operational KPI queries.
- `files`: attachments, images, invoices, labels, receipts and proof metadata.
- `settings`: versioned configuration values and recalculation impact.

Cross-module work should use explicit application services or commands rather than allowing UI pages to write directly to unrelated tables.

### 16.5 API and error behaviour

Every state-changing endpoint or command should:

- Validate the current record state and reject invalid transitions.
- Return the persisted record and any recalculated totals needed by the interface.
- Return structured field errors for forms and a stable error code for domain conflicts.
- Use optimistic-concurrency information for records that can be edited from more than one view, especially reseller requests and orders.
- Be idempotent where a browser retry could otherwise duplicate a receipt, request submission, production completion, shipment or invoice import.

List endpoints should support pagination (including a page-size parameter honouring the configurable default from Section 4.3/11.13), sorting, search and the filters owned by their page. KPI and list queries must share the same period, activity and inclusion rules so their figures cannot disagree.

### 16.6 Persistence and deletion rules

- Monetary values use decimal types; never binary floating-point.
- Durations and physical quantities use explicit units.
- Dates are stored consistently and rendered in `Europe/Paris`.
- Mutable business configuration is versioned or snapshotted when used by an order, production or invoice.
- Records referenced by history are archived, not hard-deleted.
- File blobs are stored separately from relational metadata.
- Database migrations and seed/demo data remain separate from real Acebau data.
- Every migration run is preceded by an automatic full-database snapshot (Section 3.3); migration tooling must not run against production without this step.

### 16.7 Initial repository delivery target

The first repository milestone should establish the shared domain model, database migrations, API conventions, design tokens, navigation shell and automated test harness. Feature implementation should then follow the phases in section 15. Each phase should include backend rules, frontend states, responsive behaviour and end-to-end acceptance tests before the next phase begins.

## 18. Instructions for the implementing coding agent

This section makes the concrete engineering decisions this specification otherwise leaves open, so that a coding agent (e.g. Codex) can start implementation without asking for clarification. Where a decision here conflicts with informal conventions elsewhere in the industry, this section governs for this repository.

### 18.1 Technology stack

Unless the repository already contains an established stack the agent must respect instead, use:

- **Frontend:** TypeScript, React, a component-based router (e.g. React Router or a framework router), and a design-token-driven CSS approach consistent with Section 4 (theming, contrast, responsive rules). Server-side rendering is not required for the MVP.
- **Backend:** TypeScript (Node.js) or Python, exposing a versioned HTTP JSON API (e.g. `/api/v1/...`). Pick one language for the whole backend; do not mix.
- **Database:** PostgreSQL, accessed through a typed ORM/query builder that supports explicit transactions (required by Sections 12.1 and 16.3).
- **File storage:** an object-storage abstraction (local filesystem in development, S3-compatible storage in production) behind a single interface, per Section 12.3.
- **Authentication:** server-issued, HTTP-only, `Secure`, `SameSite=Lax` session cookies (see Section 18.6). Do not implement authentication as a client-side-only guard.

If the repository already has a chosen stack that conflicts with the above, follow the existing repository stack and note the deviation in the pull request description rather than introducing a second stack.

### 18.2 Repository and environment conventions

- All configuration that differs between environments (database URL, session secret, admin bootstrap credential, base URL, file-storage credentials) is read from environment variables, never hard-coded, and never committed.
- Provide a `.env.example` listing every required variable with a placeholder value and a one-line comment.
- Database schema changes are expressed as ordered, reversible migrations, never as manual schema edits.
- Seed/demo data used for local development must be clearly separated from production data and must never run automatically against a production database (Section 16.6).

### 18.3 Testing requirements and definition of done

A phase (Section 15) or acceptance-criteria item (Section 14) is not complete until:

- The relevant domain invariant has an automated unit or integration test (e.g. stock cannot go negative, SKU uniqueness, atomic production completion).
- The relevant workflow has at least one end-to-end test covering the happy path described in this specification.
- The login flow has automated tests for: successful login, invalid credentials, rate-limiting after repeated failures, session expiry, logout, and unauthenticated access being redirected from every internal route.
- Accessibility requirements in Section 4.2 that are testable automatically (focus visibility, dialog focus trap, keyboard tab order) are covered by at least a smoke-level automated check.
- The build passes linting, type-checking and the automated test suite in CI before a phase is considered done.

### 18.4 Coding agent workflow

- Work through the phases in the order given in Section 15; do not begin Phase B work before Phase A's acceptance-criteria items (Section 14) pass.
- Treat each numbered item in Section 14 as an independent, checkable task. When an item is implemented, its corresponding automated test(s) should exist and pass.
- Keep commits/PRs scoped to one coherent unit of work (one page, one workflow, or one domain module) and describe which specification section(s) and acceptance-criteria number(s) they satisfy.
- When this specification is ambiguous or silent on a concrete detail (e.g. an exact copy string, a specific rate-limit number), choose the narrowest reasonable default, implement it behind a named constant or configuration value so it can be changed later, and record the assumption in the PR description rather than blocking on it.

### 18.5 Deployment and public availability

- The application must be deployable to a publicly reachable HTTPS URL; no requirement in this specification assumes local-only or private-network-only access.
- Every internal route (Sections 5–13) must be unreachable without a valid session, verified server-side on every request — not only hidden from navigation.
- The public reseller catalogue (Sections 8–9, 11.14) remains reachable without login, exactly as scoped in Section 8.2; it must not be placed behind the administrator login.
- Standard public-facing hardening applies: HTTPS-only, secure response headers (e.g. `Strict-Transport-Security`, `X-Content-Type-Options`, `X-Frame-Options` or an equivalent CSP `frame-ancestors`), and CSRF protection on state-changing requests that rely on cookies.

### 18.6 Authentication, session and security implementation details

- **Password storage:** hash with a modern adaptive algorithm (bcrypt or argon2) with a per-password salt; never store or log plaintext passwords.
- **Session lifetime:** default session (without "remember me") expires after 8 hours of issuance or 30 minutes of inactivity, whichever comes first; with "remember me" checked, the session lifetime extends to 30 days. These are configuration defaults, not hard-coded literals, so they can be tuned later.
- **Rate limiting:** after 5 failed login attempts for the same account within 15 minutes, block further attempts for that account for 15 minutes; independently rate-limit by source IP to slow credential-stuffing attempts. Show a clear "too many attempts, try again later" message rather than a silent failure.
- **Login bootstrapping:** the single administrator credential (e-mail + password hash) is created from environment variables or a one-time setup script at deployment time. There is no public registration endpoint.
- **Password reset (manual MVP procedure):** the administrator's password is reset by an operator with direct database or deployment access re-running the bootstrap/setup script with a new password; this is documented as an operational runbook step, not an in-app "forgot password" flow (in scope, self-service e-mail-based reset is explicitly postponed per Section 2.2).
- **Session invalidation:** logout invalidates the session server-side immediately (e.g. delete the session row or revoke the token), not merely by clearing the client-side cookie.
- **Transport:** enforce HTTPS end-to-end; cookies use the `Secure` flag so they are never sent over plain HTTP.

### 18.7 Guardrails — what not to build

To avoid scope creep beyond this MVP, the agent must not implement, even opportunistically: multiple administrator accounts or roles, self-service reseller accounts, automatic marketplace/bank/carrier synchronisation, automatic production optimisation or machine control, partial invoice payments, configurable ad hoc report exports, or any feature listed in Section 2.2. Database backup/restore (Section 3.3) is in scope and should not be skipped or treated as postponed. If a task seems to require a postponed feature, stop and treat it as a specification gap rather than building it.

## 19. Revision history

### Version 1.9 — 10 August 2026

- Translated "Dépôt-vente" to "Consignment stock management" (Section 2.2).
- Moved database backup/export/import into MVP scope: added Section 3.3 (manual export, automatic pre-migration snapshot, confirmed restore), a Settings "Data" section, a Section 16.6 persistence rule, and updated the Section 18.7 guardrails accordingly.
- Added a short e-mailed access code (4 characters, letters/digits) that the reseller enters alongside their private catalogue link, so the link and the code together identify the reseller (Section 8.2); propagated the change through Sections 2.1, 3.2, 8.1, 11.10/11.11, 16.4 and 18.
- Added modern visual-language guidance to Section 4.2 (white space, typography scale, soft elevation, rounded corners, restrained colour, subtle motion) without weakening existing accessibility/contrast requirements.
- Made the table page size configurable (default 10) instead of fixed, with a global Settings default and a per-table override (Sections 4.3, 11.13, 16.5).
- Added acceptance-criteria items 31–33 covering backup/restore, the reseller access code and the configurable table page size.

### Version 1.8 — 10 August 2026

- Removed all references to a named individual; the sole administrator role is now referred to generically throughout ("the administrator").
- Changed the deployment model from a private local/network-only application to a publicly reachable HTTPS deployment.
- Added a mandatory administrator login page (Section 3.1.1) gating every internal module; removed the prior "opens without a sign-in screen" behaviour.
- Updated Section 2 (MVP boundaries) so authentication is in scope for the MVP, while multiple accounts/roles and self-service password reset remain explicitly postponed.
- Added Section 18 ("Instructions for the implementing coding agent") with concrete stack, environment, testing, deployment, session/security and guardrail decisions so the specification is directly actionable by an autonomous coding agent without further clarification.
- Renumbered the former Section 17 ("Revision history") to Section 19.

### Version 1.7 — 10 August 2026

- Declared this specification, rather than any generated preview, as the repository source of truth.
- Consolidated the final page-by-page UX decisions and removed remaining contradictions.
- Made KPI cards consistently non-interactive and removed redundant `Open` table actions.
- Clarified that machine profiles store estimated filament mass, while recipe and production choose the material/colour and physical spool.
- Finalised machine KPIs, printed-piece list content, one-shipment/multi-parcel behaviour, manual production authorisation and editable Pending reseller requests.
- Removed credit-note management from the MVP.
- Added frontend/backend responsibilities, domain boundaries, transactional rules, API behaviour and persistence guidance for repository implementation.

### Version 1.6 — 8 August 2026

- Used the complete v1.5 preview as the visual and page baseline for the final UX review.
- Restored reseller detail and made list-to-detail navigation and reseller actions operational.
- Added production incident filtering, operational task updates, production history and planner actions while preserving the existing machine timeline.
- Completed simple machine detail, seven-day load, spool creation, catalogue/product/variant workflows and the seven-step product/variant assistant.
- Added order lifecycle distribution, nested order contents, manual production authorisation, custom quoting and one shipment with multiple parcels.
- Completed dependent public variant selection, configured basket lines, request confirmation and editable Pending-request revisions.
- Aligned KPI formatting, settings safeguards and the specification with the validated v1.6 interactions.

### Version 1.2 — 8 August 2026

- Reorganised internal navigation around Pilotage, Atelier, Offre, Ventes, Analyse and Configuration.
- Added keyboard, focus, dialog, mobile-table, form-feedback and long-page navigation requirements.
- Aligned page KPIs with operational decisions and made dashboard figures consistent with Finance and Données.
- Clarified the order lifecycle, public catalogue filters, basket calculations, production timeline and historical-finance workflow.
- Added UX-specific acceptance criteria for all primary interactions on desktop and mobile.

### Version 1.1 — 8 August 2026

- Audited every design page against the MVP specification.
- Reworked page KPIs around decisions, exceptions and deadlines.
- Added dedicated Printed pieces management.
- Added the Data and commercial-performance module.
- Expanded Finance with monthly and annual turnover tables, activity split and cash-position information.
- Completed production subviews, order lifecycle visibility, reseller detail, invoice exceptions and inventory counts.
