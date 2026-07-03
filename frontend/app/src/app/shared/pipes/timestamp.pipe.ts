import { Pipe, PipeTransform } from '@angular/core';
import { format, fromUnixTime } from 'date-fns';
import { es } from 'date-fns/locale';

@Pipe({ name: 'ts', standalone: true })
export class TimestampPipe implements PipeTransform {
  transform(value: number | null | undefined): string {
    if (value == null) return '—';
    try {
      return format(fromUnixTime(value), 'dd MMM yyyy', { locale: es });
    } catch {
      return String(value);
    }
  }
}
