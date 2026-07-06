import { getHome } from '../../api/home.api';
import type { HomeView } from '../view-models';

export class HomeService {
  static async load(): Promise<HomeView> {
    return getHome();
  }
}
