import { HomeView } from '../core/view-models';
const BASE = 'http://localhost:8000';

export async function getHome(): Promise<HomeView> {
  const r = await fetch(`${BASE}/api/home`);
  if (!r.ok) throw new Error(`home ${r.status}`);
  return r.json();
}
