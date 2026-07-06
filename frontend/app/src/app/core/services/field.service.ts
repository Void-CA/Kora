import { getFields, getField } from '../../api/field.api';
import type { Field } from '../../api/field.api';

export class FieldService {
  static async list(): Promise<Field[]> { return getFields(); }
  static async get(id: string): Promise<Field> { return getField(id); }
}
