import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
    const res = await fetch('http://localhost:8080/printing_environments');
    const items = await res.json();

    console.log(items);
    return { items: items };
};
