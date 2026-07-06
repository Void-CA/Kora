import { getCycles, getCycle, getCycleVariance, getCycleTimeline, getProfitability, registerActivity } from '../../api/cycle.api';

export class CycleService {
  static async list() { return getCycles(); }
  static async get(id: string) { return getCycle(id); }
  static async variance(id: string) { return getCycleVariance(id); }
  static async timeline(id: string) { return getCycleTimeline(id); }
  static async profitability(id: string) { return getProfitability(id); }
  static async registerActivity(cycleId: string, body: any) { return registerActivity(cycleId, body); }
}
