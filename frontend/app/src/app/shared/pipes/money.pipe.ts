import { Pipe, PipeTransform } from '@angular/core';

@Pipe({ name: 'money', standalone: true })
export class MoneyPipe implements PipeTransform {
  transform(value: string | null | undefined): string {
    if (!value) return '—';
    // Backend format: "150 USD", "50.00 NIO", etc.
    const parts = value.split(' ');
    const amount = parts[0] ?? '';
    const currency = parts[1] ?? '';
    const symbol = currency === 'USD' ? '$' : currency === 'NIO' ? 'C$' : currency;
    try {
      const n = parseFloat(amount);
      if (isNaN(n)) return value;
      return `${symbol}${n.toLocaleString('es')}`;
    } catch {
      return value;
    }
  }
}
